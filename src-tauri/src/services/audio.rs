use lofty::ItemKey;
use lofty::{
    Accessor, AudioFile, MimeType, ParseOptions, Picture, PictureType, Probe, TagExt, TaggedFileExt,
};
use std::path::Path;

use crate::models::{AppError, Track};

pub struct AudioService;

impl Default for AudioService {
    fn default() -> Self {
        Self::new()
    }
}

impl AudioService {
    pub fn new() -> Self {
        Self
    }

    pub fn lire_metadonnees(&self, chemin: &str) -> Result<Track, AppError> {
        let path = Path::new(chemin);
        let filename = path
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();

        // Utiliser Probe pour détecter et lire le fichier
        let tagged_file = Probe::open(path)
            .map_err(|e| AppError::Audio(format!("Erreur d'ouverture: {}", e)))?
            .options(ParseOptions::new().read_properties(true))
            .read()
            .map_err(|e| AppError::Audio(format!("Erreur de lecture: {}", e)))?;

        let tag = tagged_file.primary_tag();
        let properties = tagged_file.properties();

        let mut track = Track::new(chemin.to_string(), filename);

        // Récupérer la taille du fichier
        if let Ok(metadata) = std::fs::metadata(path) {
            track.size = metadata.len();
        }

        // Remplir les propriétés audio
        track.duration_sec = properties.duration().as_secs();
        track.bit_rate = properties.audio_bitrate();
        // track.format = ... (Lofty ne donne pas directement le format sous forme de string simple, on peut déduire de l'extension ou du FileType)
        if let Some(ext) = path.extension() {
            track.format = ext.to_string_lossy().to_string().to_lowercase();
        }

        // Remplir les métadonnées si un tag existe
        if let Some(tag) = tag {
            track.title = tag.title().unwrap_or_default().to_string();
            track.artist = tag.artist().unwrap_or_default().to_string();
            track.album_artist = tag
                .get_string(&ItemKey::AlbumArtist)
                .unwrap_or_default()
                .to_string();
            track.album = tag.album().unwrap_or_default().to_string();
            track.year = tag.year();
            track.track_number = tag.track();
            track.genre = tag.genre().map(|s| s.to_string());
            track.has_cover = tag.picture_count() > 0;
        }

        // Sauvegarder les métadonnées originales pour la comparaison
        track.original_metadata = Some(Box::new(track.clone()));

        Ok(track)
    }

    pub fn ecrire_metadonnees(&self, track: &Track) -> Result<(), AppError> {
        let path = Path::new(&track.path);

        let mut tagged_file = Probe::open(path)
            .map_err(|e| AppError::Audio(format!("Erreur d'ouverture: {}", e)))?
            .read()
            .map_err(|e| AppError::Audio(format!("Erreur de lecture: {}", e)))?;

        let tag = match tagged_file.primary_tag_mut() {
            Some(primary_tag) => primary_tag,
            None => {
                let first_tag_type = tagged_file.file_type().primary_tag_type();
                tagged_file.insert_tag(lofty::Tag::new(first_tag_type));
                tagged_file.primary_tag_mut().unwrap()
            }
        };

        tag.set_title(track.title.clone());
        tag.set_artist(track.artist.clone());
        tag.set_album(track.album.clone());
        tag.insert_text(ItemKey::AlbumArtist, track.album_artist.clone());
        if let Some(ref genre) = track.genre {
            tag.set_genre(genre.clone());
        }
        if let Some(year) = track.year {
            tag.set_year(year);
        }
        if let Some(track_num) = track.track_number {
            tag.set_track(track_num);
        }

        tag.save_to_path(path)
            .map_err(|e| AppError::Audio(format!("Erreur d'écriture: {}", e)))?;

        Ok(())
    }
    pub fn definir_cover(&self, track_path: &str, cover_path: &str) -> Result<(), AppError> {
        let path = Path::new(track_path);

        let mut tagged_file = Probe::open(path)
            .map_err(|e| AppError::Audio(format!("Erreur d'ouverture: {}", e)))?
            .read()
            .map_err(|e| AppError::Audio(format!("Erreur de lecture: {}", e)))?;

        let tag = match tagged_file.primary_tag_mut() {
            Some(primary_tag) => primary_tag,
            None => {
                let first_tag_type = tagged_file.file_type().primary_tag_type();
                tagged_file.insert_tag(lofty::Tag::new(first_tag_type));
                tagged_file.primary_tag_mut().unwrap()
            }
        };

        // Lire le fichier image
        let img_data = std::fs::read(cover_path)
            .map_err(|e| AppError::Audio(format!("Erreur lecture cover: {}", e)))?;

        // Créer l'objet Picture
        // On assume JPEG car le CoverService convertit tout en JPEG
        let picture =
            Picture::new_unchecked(PictureType::CoverFront, MimeType::Jpeg, None, img_data);

        // Supprimer les anciennes covers
        tag.remove_picture_type(PictureType::CoverFront);

        // Ajouter la nouvelle
        tag.push_picture(picture);

        tag.save_to_path(path)
            .map_err(|e| AppError::Audio(format!("Erreur sauvegarde tags: {}", e)))?;

        Ok(())
    }
}
