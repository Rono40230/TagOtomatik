import { defineStore } from 'pinia'
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { useToastStore } from './toast'

export interface CaseException {
  id?: number
  original: string
  corrected: string
  category: 'artist' | 'album' | 'global'
}

export const useExceptionStore = defineStore('exceptions', () => {
  const exceptions = ref<CaseException[]>([])
  const loading = ref(false)
  const error = ref<string | null>(null)
  const toast = useToastStore()

  async function chargerExceptions() {
    loading.value = true
    error.value = null
    try {
      exceptions.value = await invoke('get_exceptions')
    } catch (e: unknown) {
      const errMsg = e instanceof Error ? e.message : String(e)
      error.value = errMsg
      toast.error(`Erreur chargement exceptions: ${errMsg}`)
    } finally {
      loading.value = false
    }
  }

  async function ajouterException(original: string, corrected: string, category: string) {
    loading.value = true
    error.value = null
    try {
      const newException = await invoke<CaseException>('add_exception', {
        original,
        corrected,
        category
      })
      // Mettre à jour la liste locale ou recharger
      // On vérifie si c'est un remplacement ou un ajout
      const index = exceptions.value.findIndex(e => e.id === newException.id)
      if (index !== -1) {
        exceptions.value[index] = newException
        toast.success('Exception mise à jour.')
      } else {
        exceptions.value.push(newException)
        toast.success('Exception ajoutée.')
      }
    } catch (e: unknown) {
      const errMsg = e instanceof Error ? e.message : String(e)
      error.value = errMsg
      toast.error(`Erreur ajout exception: ${errMsg}`)
    } finally {
      loading.value = false
    }
  }

  async function supprimerException(id: number) {
    loading.value = true
    error.value = null
    try {
      await invoke('delete_exception', { id })
      exceptions.value = exceptions.value.filter(e => e.id !== id)
      toast.success('Exception supprimée.')
    } catch (e: unknown) {
      const errMsg = e instanceof Error ? e.message : String(e)
      error.value = errMsg
      toast.error(`Erreur suppression exception: ${errMsg}`)
    } finally {
      loading.value = false
    }
  }

  return {
    exceptions,
    loading,
    error,
    chargerExceptions,
    ajouterException,
    supprimerException
  }
})
