export interface Track {
    path: string;
    filename: string;
    title: string;
    artist: string;
    album_artist: string;
    album: string;
    year?: number;
    track_number?: number;
    genre?: string;
    duration_sec: number;
    format: string;
    bit_rate?: number;    size: number;    has_cover: boolean;
    is_modified: boolean;
    original_metadata?: Track;
}

export enum AlbumStatus {
    Clean = "Clean",
    Dirty = "Dirty",
    Processing = "Processing",
    Incomplete = "Incomplete",
}

export interface Album {
    id: string;
    path: string;
    title: string;
    artist: string;
    year?: number;
    yearMin?: number;
    yearMax?: number;
    cover_path?: string;
    has_playlist: boolean;
    tracks: Track[];
    status: AlbumStatus;
    issues?: string[];
}

export interface MusicBrainzRelease {
    id: string;
    title: string;
    'artist-credit'?: { name: string }[];
    date?: string;
    country?: string;
}

export interface ScanResult {
    albums: Album[];
    errors: string[];
}
