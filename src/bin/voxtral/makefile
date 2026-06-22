
# ce message est synthétisé en 1'24" soit 3 x la durée de l'audio, mais du silence est généré ;)
MESSAGE = "Quand un simple besoin amène, de proche en proche, à explorer beaucoup de choses. Le soir, j'aime bien écouter des bouquins, lire me fatigue. J'utilise donc des audio books, mais un audio book , c'est cher et j'ai déjà pas mal de bouquins en textes, ( ebook ). j'aimerais les convertir en audio. Mistral a justement sorti un balaise de modele récent qui bat la concurrence en TTS. j'ai essayé, c'est bluffant."



X= "Probleme : la RAM de ma carte ( 12G de VRAM ) est trop petite, il en faut au moins 16 . Je fais un essai sur le cloud : j'ai loué une instance linux avec GPU sur Scaleway. ça marche, mais le prix du calcul pour convertir un bouquin revient au même prix que l'audiobook chez Audible ;( . Je trouve un bienfaiteur qui a comprimé le modele Voxtral  : les 4 millions de coefs du modele sont quantifiés ( quantif vectorielle) , l'expansion des coef est faite a la volée sur le gpu.  on perd quasiment rien en qualité. là, ça peut rentrer dans ma GPU"

start :
# Synthesize speech (BF16 or Q4)
	cd ../../..; RUST_LOG=info cargo run --release --features "wgpu,cli,hub" --bin voxtral -- speak --text $(MESSAGE) --voice fr_female --gguf models/voxtral-tts-q4.gguf --output out_gguf.wav --euler-steps 8

