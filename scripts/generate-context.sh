#!/bin/bash
# generate-context.sh - Génère une carte du projet pour l'IA
# Crée un fichier PROJECT_MAP.md qui résume la structure et les interfaces publiques

OUTPUT_FILE="PROJECT_MAP.md"

echo "# 🗺️ PROJECT MAP (Généré automatiquement)" > "$OUTPUT_FILE"
echo "Date: $(date)" >> "$OUTPUT_FILE"
echo "" >> "$OUTPUT_FILE"

echo "## 📂 Arborescence (Backend)" >> "$OUTPUT_FILE"
echo '```' >> "$OUTPUT_FILE"
find src-tauri/src -maxdepth 3 -not -path '*/.*' | sort | sed 's/[^/]*\//  /g' >> "$OUTPUT_FILE"
echo '```' >> "$OUTPUT_FILE"

echo "" >> "$OUTPUT_FILE"
echo "## 📂 Arborescence (Frontend)" >> "$OUTPUT_FILE"
echo '```' >> "$OUTPUT_FILE"
find src -maxdepth 3 -not -path '*/.*' | sort | sed 's/[^/]*\//  /g' >> "$OUTPUT_FILE"
echo '```' >> "$OUTPUT_FILE"

echo "" >> "$OUTPUT_FILE"
echo "## 🦀 Interfaces Publiques (Rust)" >> "$OUTPUT_FILE"
echo "Liste des fonctions publiques et structs exposés :" >> "$OUTPUT_FILE"
echo '```rust' >> "$OUTPUT_FILE"
# Cherche les 'pub fn', 'pub struct', 'pub enum' dans src-tauri
grep -rE "^pub (fn|struct|enum|type|trait)" src-tauri/src | sed 's/src-tauri\/src\///' >> "$OUTPUT_FILE"
echo '```' >> "$OUTPUT_FILE"

echo "✅ PROJECT_MAP.md mis à jour."
