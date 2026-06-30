.PHONY: build build-release build-wasm lint lint-wasm fmt test bench bench-audio bench-q4 bench-e2e profile-chrome profile-flamegraph eval-wer-fleurs eval-wer-libri clean

# Build
build:
	cargo build --features "wgpu,cli,hub"

build-release:
	cargo build --release --features "wgpu,cli,hub"

build-wasm:
	wasm-pack build --target web --no-default-features --features wasm

# Lint & Format
lint:
	cargo clippy --features "wgpu,cli,hub" -- -D warnings

lint-wasm:
	cargo clippy --no-default-features --features wasm --target wasm32-unknown-unknown -- -D warnings

fmt:
	cargo fmt

fmt-check:
	cargo fmt -- --check

# Test
test:
	cargo test --features "wgpu,cli,hub"

# Benchmarks
bench-audio:
	cargo bench --bench audio

bench-q4:
	cargo bench --bench q4_ops --features wgpu

bench: bench-audio bench-q4

bench-e2e:
	cargo run --release --features "wgpu,cli" --bin e2e-bench -- \
		--audio test_data/mary_had_lamb.wav \
		--gguf models/voxtral-q4.gguf \
		--tokenizer models/voxtral/tekken.json

# Profiling
profile-chrome:
	cargo run --profile profiling --features "wgpu,cli,profiling" --bin voxtral-transcribe -- \
		--audio test_data/mary_had_lamb.wav \
		--gguf models/voxtral-q4.gguf \
		--tokenizer models/voxtral/tekken.json
	@echo "Trace written to trace.json — open in chrome://tracing or https://ui.perfetto.dev"

profile-flamegraph:
	cargo flamegraph --features "wgpu,cli" --bin voxtral-transcribe -- \
		--audio test_data/mary_had_lamb.wav \
		--gguf models/voxtral-q4.gguf \
		--tokenizer models/voxtral/tekken.json

# WER Evaluation
eval-wer-fleurs:
	uv run --script scripts/eval_wer.py -- \
		--dataset fleurs \
		--gguf models/voxtral-q4.gguf \
		--tokenizer models/voxtral/tekken.json \
		--delay 6

eval-wer-libri:
	uv run --script scripts/eval_wer.py -- \
		--dataset librispeech-clean \
		--gguf models/voxtral-q4.gguf \
		--tokenizer models/voxtral/tekken.json \
		--delay 6

# Cleanup
clean:
	cargo clean
	rm -rf pkg/ trace.json flamegraph.svg perf.data perf.data.old

begin : download convert_voice_models list-voices

download :
# Download TTS model weights (~8 GB BF16 or ~2.67 GB Q4)
	uv run --with huggingface_hub \
	hf download mistralai/Voxtral-4B-TTS-2603 --local-dir models/voxtral-tts
	uv run --with huggingface_hub hf download TrevorJS/voxtral-tts-q4-gguf voxtral-tts-q4.gguf --local-dir models

convert_voice_models :
	python scripts/convert_voice_embeds.py models/voxtral-tts/voice_embedding

synthesis :
# Synthesize speech (BF16 or Q4)
	cargo run --release --features "wgpu,cli,hub" --bin voxtral -- speak --text "Hello world" --voice casual_female
	cargo run --release --features "wgpu,cli,hub" --bin voxtral -- speak --text "Hello world" --voice casual_female --gguf models/voxtral-tts-q4.gguf

# ce message est synthétisé en 1'24" soit 3 x la durée de l'audio, mais du silence est généré ;)
MESSAGE = "quand un simple besoin amène, de proche en proche,  à explorer beaucoup de choses. le soir, j'aime bien écouter des bouquins, lire me fatigue. j'utilise donc des audio books mais un audio book , c'est cher et j'ai déjà pas mal de bouquins en textes, ( ebook). j'aimerais les convertir en audio. Mistral a justement sorti un balaise de modele récent qui bat la concurrence en TTS. j'ai essayé, c'est bluffant  : Voxtral TTS ( c'est dispo en 'open weights' sur hummingbird ). Je me dit, je vais l'utiliser sur ma petite carte GPU pour me créer mon audioteque. "

# calcul : 56", durée message : 17"
MESSAGE = "Salut tout le monde, c'est voxtral qui vous parle avec ma belle voix articifielle! Qu'en pensez-vous?"

# ce message est synthétisé en 1'24" soit 3 x la durée de l'audio, mais du silence est généré ;)
MESSAGE = "Quand un simple besoin amène, de proche en proche, à explorer beaucoup de choses. Le soir, j'aime bien écouter des bouquins, lire me fatigue. J'utilise donc des audio books, mais un audio book , c'est cher et j'ai déjà pas mal de bouquins en textes, ( ebook ). j'aimerais les convertir en audio. Mistral a justement sorti un balaise de modele récent qui bat la concurrence en TTS. j'ai essayé, c'est bluffant."



X= "Probleme : la RAM de ma carte ( 12G de VRAM ) est trop petite, il en faut au moins 16 . Je fais un essai sur le cloud : j'ai loué une instance linux avec GPU sur Scaleway. ça marche, mais le prix du calcul pour convertir un bouquin revient au même prix que l'audiobook chez Audible ;( . Je trouve un bienfaiteur qui a comprimé le modele Voxtral  : les 4 millions de coefs du modele sont quantifiés ( quantif vectorielle) , l'expansion des coef est faite a la volée sur le gpu.  on perd quasiment rien en qualité. là, ça peut rentrer dans ma GPU"

synthesis_fr :
# Synthesize speech (BF16 or Q4)
	RUST_LOG=info cargo run --release --features "wgpu,cli,hub" --bin voxtral -- speak --text $(MESSAGE) --voice fr_female --gguf models/voxtral-tts-q4.gguf --output out_gguf.wav

synthesize_fr_2 :
	-cargo run --release --features "wgpu,cli,hub" --bin voxtral -- speak --text $(MESSAGE) --voice fr_female --output out.wav

# Real-time with 3 Euler steps
	-cargo run --release --features "wgpu,cli,hub" --bin voxtral -- speak --text $(MESSAGE) --voice fr_female --gguf models/voxtral-tts-q4.gguf --euler-steps 3 --output out_gguf_euler.wav

list-voices :
# List available voices
	cargo run --release --features "wgpu,cli,hub" --bin voxtral -- speak --list-voices

quantize :
	uv run --with safetensors --with torch --with numpy --with packaging scripts/quantize_tts_gguf.py models/voxtral-tts/ -o voxtral-tts-q4.gguf

start :
	cargo run --bin essai
