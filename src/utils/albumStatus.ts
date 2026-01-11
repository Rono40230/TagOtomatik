import type { Album } from '../types';

export function getAlbumStatusTooltip(album: Album): string {
    if (album.status === 'Clean') return 'Album complet et conforme';
    if (album.status === 'Processing') return 'Analyse en cours...';

    // Use backend issues if available (Source of Truth)
    if (album.issues && album.issues.length > 0) {
        return album.issues.map(i => `• ${i}`).join('\n');
    }

    // Fallback for legacy/transition state/frontend calc
    const issues: string[] = [];

    if (!album.cover_path) issues.push('• Cover manquante');
    if (!album.has_playlist) issues.push('• Playlist manquante');
    if (!album.year || album.year === 0) issues.push('• Année manquante');

    let missingTitles = 0;
    let missingArtists = 0;
    let missingAlbums = 0;
    let missingGenres = 0;

    album.tracks.forEach(t => {
        if (!t.title || t.title.trim() === '') missingTitles++;
        if (!t.artist || t.artist.trim() === '') missingArtists++;
        if (!t.album || t.album.trim() === '') missingAlbums++;
        if (!t.genre || t.genre.trim() === '') missingGenres++;
    });

    if (missingTitles > 0) issues.push(`• Titre manquant (${missingTitles} pistes)`);
    if (missingArtists > 0) issues.push(`• Artiste manquant (${missingArtists} pistes)`);
    if (missingAlbums > 0) issues.push(`• Album manquant (${missingAlbums} pistes)`);
    if (missingGenres > 0) issues.push(`• Genre manquant (${missingGenres} pistes)`);

    if (album.tracks.length === 0) issues.push('• Aucune piste audio');

    return issues.length > 0 ? issues.join('\n') : 'Statut inconnu';
}
