
# ce message est synthétisé en 1'24" soit 3 x la durée de l'audio, mais du silence est généré ;)
MESSAGE = "Quand un simple besoin amène, de proche en proche, à explorer beaucoup de choses. Le soir, j'aime bien écouter des bouquins, lire me fatigue. J'utilise donc des audio books, mais un audio book , c'est cher et j'ai déjà pas mal de bouquins en textes, ( ebook ). j'aimerais les convertir en audio. Mistral a justement sorti un balaise de modele récent qui bat la concurrence en TTS. j'ai essayé, c'est bluffant."


BIG_MESSAGE = "Quand un simple besoin amène, de proche en proche, à explorer beaucoup de choses. Le soir, j'aime bien écouter des bouquins, lire me fatigue. J'utilise donc des audio books, mais un audio book , c'est cher et j'ai déjà pas mal de bouquins en textes, ( ebook ). j'aimerais les convertir en audio. Mistral a justement sorti un balaise de modele récent qui bat la concurrence en TTS. j'ai essayé, c'est bluffant. Probleme: la RAM de ma carte ( 12 Giga de VRAM ) est trop petite, il en faut au moins 16 . J'ai fait un essai sur le cloud : j'ai loué une instance linux avec GPU sur Scaleway. ça marche, mais le prix du calcul pour convertir un bouquin revient au même prix que l'audiobook chez Audible . J'ai enfin trouvé un bienfaiteur qui a comprimé le modèle Voxtral : les 4 millions de coefs du modèle sont quantifiés ( quantif vectorielle) , l'expansion des coef est faite a la volée sur le gpu.  on perd quasiment rien en qualité. là, ça peut rentrer dans ma GPU !"

ENORMOUS_MESSAGE = $(BIG_MESSAGE)$(BIG_MESSAGE)

MEDIUM_MESSAGE= "Quand un simple besoin amène, de proche en proche, à explorer beaucoup de choses. Le soir, j'aime bien écouter des bouquins, lire me fatigue. J'utilise donc des audio books, mais un audio book , c'est cher et j'ai déjà pas mal de bouquins en textes, ( des ebooks ). j'aimerais les convertir en audio. Mistral a justement sorti un balaise de modele récent qui bat la concurrence en TTS. j'ai essayé, c'est bluffant. Problème: la RAM de ma carte ( 12 Giga de VRAM ) est trop petite, il en faut au moins 16."

SMALL_MESSAGE= "Quand un simple besoin amène, de proche en proche, à explorer beaucoup de choses."

#start : quant


eee :
	echo $(ENORMOUS_MESSAGE)

start : bonjour

messages  :
# Synthesize speech (BF16 or Q4)
#	RUST_LOG=info cargo run --manifest-path=../../../Cargo.toml --release --features "wgpu,cli,hub" --bin voxtral -- speak --text $(MESSAGE) --voice fr_female --gguf models/voxtral-tts-q4.gguf --output /mnt/NUC/data/out_gguf_8.wav --euler-steps 8
#	RUST_LOG=info cargo run --manifest-path=../../../Cargo.toml --release --features "wgpu,cli,hub" --bin voxtral -- speak --text $(MESSAGE) --voice fr_female --gguf models/voxtral-tts-q4.gguf --output /mnt/NUC/data/out_gguf.wav 
	-cd ../../..; RUST_LOG=info cargo  run --release --features "wgpu,cli,hub" --bin voxtral -- speak --text $(BIG_MESSAGE) --voice fr_female --gguf models/voxtral-tts-q4.gguf --output /mnt/NUC/data/out_gguf_big_female_$(TAG).wav
	cd ../../..; RUST_LOG=info cargo  run --release --features "wgpu,cli,hub" --bin voxtral -- speak --text $(MEDIUM_MESSAGE) --voice fr_female --gguf models/voxtral-tts-q4.gguf --output /mnt/NUC/data/out_gguf_medium_female_$(TAG).wav
	cd ../../..; RUST_LOG=info cargo  run --release --features "wgpu,cli,hub" --bin voxtral -- speak --text $(MEDIUM_MESSAGE) --voice fr_male --gguf models/voxtral-tts-q4.gguf --output /mnt/NUC/data/out_gguf_medium_male_$(TAG).wav 
	-cd ../../..; RUST_LOG=info cargo  run --release --features "wgpu,cli,hub" --bin voxtral -- speak --text $(BIG_MESSAGE) --voice fr_male --gguf models/voxtral-tts-q4.gguf --output /mnt/NUC/data/out_gguf_big_male_$(TAG).wav
#	cd ../../..; RUST_LOG=info cargo run --release --features "wgpu,cli,hub" --bin voxtral -- speak --text $(ENORMOUS_MESSAGE) --voice fr_male --gguf models/voxtral-tts-q4.gguf --output /mnt/NUC/data/out_gguf_enormous_male.wav

bonjour :
#	cd ../../..; RUST_LOG=info cargo  run --features "wgpu,cli,hub" --bin voxtral -- speak --text "Bonjour monsieur" --voice fr_female --gguf models/voxtral-tts-q4.gguf --output /mnt/NUC/data/out_gguf_bonjour1.wav
#	-cd ../../..; RUST_LOG=info cargo  run --release --features "wgpu,cli,hub" --bin voxtral -- speak --text "Bonjour monsieur!" --voice fr_female --gguf models/voxtral-tts-q4.gguf --output /mnt/NUC/data/out_gguf_bonjour_female2.wav
#	-cd ../../..; RUST_LOG=info cargo  run --release --features "wgpu,cli,hub" --bin voxtral -- speak --text "Bonjour ! monsieur" --voice fr_female --gguf models/voxtral-tts-q4.gguf --output /mnt/NUC/data/out_gguf_bonjour_female1.wav
	-cd ../../..; RUST_LOG=info cargo  run --release --features "wgpu,cli,hub" --bin voxtral -- speak --text "Bonjour ! monsieur" --voice final_codes --gguf models/voxtral-tts-q4.gguf --output /mnt/NUC/data/out_gguf_bonjour_final.wav
#	-cd ../../..; RUST_LOG=info cargo  run --release --features "wgpu,cli,hub" --bin voxtral -- speak --text "Bonjour ! monsieur" --voice lutringer --gguf models/voxtral-tts-q4.gguf --output /mnt/NUC/data/out_gguf_bonjour_lutringer.wav 
#	-cd ../../..; RUST_LOG=info cargo  run --release --features "wgpu,cli,hub" --bin voxtral -- speak --text "Bonjour ! monsieur" --voice noiret --gguf models/voxtral-tts-q4.gguf --output /mnt/NUC/data/out_gguf_bonjour_noiret.wav


quant :
	cd ../../../scripts; python quantize_tts_gguf.py ../models/voxtral-tts

help :
	-cd ../../..; RUST_LOG=info cargo  run --release --features "wgpu,cli,hub" --bin voxtral -- speak --help

list-voices :
	cd ../../..; cargo run --release --features "wgpu,cli,hub" --bin voxtral -- speak --list-voices
