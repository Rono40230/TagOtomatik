import { defineStore } from 'pinia';
import { ref, watch } from 'vue';

export interface ConversionSettings {
  format: 'mp3' | 'flac';
  bitrate: '128k' | '192k' | '320k' | 'V0' | 'V2';
  normalize: boolean;
}

export interface PlaylistSettings {
  defaultNamePattern: string;
  useRelativePaths: boolean;
  autoCreate: boolean;
}

export const useSettingsStore = defineStore('settings', () => {
  // State
  const conversion = ref<ConversionSettings>({
    format: 'mp3',
    bitrate: '320k',
    normalize: false,
  });

  const playlist = ref<PlaylistSettings>({
    defaultNamePattern: '{artist} - {album}.m3u',
    useRelativePaths: true,
    autoCreate: false,
  });

  // Load from localStorage on init
  const savedSettings = localStorage.getItem('tagotomatik_settings');
  if (savedSettings) {
    try {
      const parsed = JSON.parse(savedSettings);
      if (parsed.conversion) conversion.value = { ...conversion.value, ...parsed.conversion };
      if (parsed.playlist) playlist.value = { ...playlist.value, ...parsed.playlist };
    } catch (e) {
      // Failed to load settings, using defaults
    }
  }

  // Auto-save watcher
  watch(
    [conversion, playlist],
    () => {
      localStorage.setItem(
        'tagotomatik_settings',
        JSON.stringify({
          conversion: conversion.value,
          playlist: playlist.value,
        })
      );
    },
    { deep: true }
  );

  return {
    conversion,
    playlist,
  };
});
