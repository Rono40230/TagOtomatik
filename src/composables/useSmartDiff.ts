import { ref } from 'vue';

export function useSmartDiff(emit: (event: 'suggest-exception', payload: { original: string; corrected: string; category: string }) => void) {
  const originalValueOnFocus = ref('');

  function onFocus(value: string | number) {
    originalValueOnFocus.value = String(value || '');
  }

  function getSmartDiff(s1: string, s2: string) {
    if (s1 === s2) return null;
    
    // 1. Find Common Prefix
    let start = 0;
    while (start < s1.length && start < s2.length && s1[start] === s2[start]) {
      start++;
    }
    
    // 2. Find Common Suffix
    let end1 = s1.length;
    let end2 = s2.length;
    
    while (end1 > start && end2 > start && s1[end1 - 1] === s2[end2 - 1]) {
      end1--;
      end2--;
    }
    
    // Helper: is word char? (Alphanumeric + Accents + Underscore)
    const isWordChar = (c: string) => /[a-zA-Z0-9À-ÿ_]/.test(c);
    
    // 3. Expand Left (Backtrack to start of word)
    let expandedStart = start;
    while (expandedStart > 0 && isWordChar(s1[expandedStart - 1])) {
      expandedStart--;
    }
    
    // 4. Expand Right (Forward to end of word)
    let expandedEnd1 = end1;
    while (expandedEnd1 < s1.length && isWordChar(s1[expandedEnd1])) {
      expandedEnd1++;
    }
    
    let expandedEnd2 = end2;
    while (expandedEnd2 < s2.length && isWordChar(s2[expandedEnd2])) {
      expandedEnd2++;
    }
    
    const diff1 = s1.substring(expandedStart, expandedEnd1);
    const diff2 = s2.substring(expandedStart, expandedEnd2);
    
    // Safety: If expansion resulted in empty strings (shouldn't happen if s1!=s2), fallback
    if (!diff1 && !diff2) return { original: s1, corrected: s2 };
    
    return { original: diff1, corrected: diff2 };
  }

  function onBlur(field: string, newValue: string | number) {
    const strNew = String(newValue || '');
    if (strNew && originalValueOnFocus.value && strNew !== originalValueOnFocus.value) {
      const diff = getSmartDiff(originalValueOnFocus.value, strNew);
      if (diff) {
        emit('suggest-exception', {
          original: diff.original,
          corrected: diff.corrected,
          category: field
        });
      }
    }
  }

  return {
    onFocus,
    onBlur
  };
}
