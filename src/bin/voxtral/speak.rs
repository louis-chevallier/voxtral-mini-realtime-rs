//! `voxtral speak` subcommand — text-to-speech.

use std::fs;
use anyhow::{bail, Context, Result};
use burn::backend::Wgpu;
use burn::tensor::{s, Tensor};
use std::path::Path;

use clap::Parser;
use std::path::PathBuf;
use std::time::Instant;
use std::iter::zip;

use tracing::info;
//use tracing_subscriber;

use voxtral_mini_realtime::audio::AudioBuffer;
use voxtral_mini_realtime::tokenizer::TekkenEncoder;

//use log::{LevelFilter};
//use std::io::Write;

use voxtral_mini_realtime::EKO;

use sentencex::segment;
use words_count;

//use burn::tensor::{Tensor, s};
//use burn::tensor::{Tensor};


//use voxtral_mini_realtime::gguf::Q4VoxtralModel;
use voxtral_mini_realtime::gguf::Q4TtsModelLoader;


type Backend = Wgpu;

#[derive(Parser)]
pub struct Args {
    /// Text to synthesize.
    #[arg(short, long, required_unless_present_any = ["list_voices", "token_ids", "file_text"])]
    text: Option<String>,

    #[arg(short, long, required_unless_present_any = ["list_voices", "token_ids", "text"])]
    file_text: Option<String>,

    /// Pre-tokenized token IDs (comma-separated, bypasses tokenizer).
    #[arg(long, value_delimiter = ',')]
    token_ids: Option<Vec<u32>>,

    /// BF16 SafeTensors model directory.
    #[arg(
        short,
        long,
        default_value = "models/voxtral-tts",
        conflicts_with = "gguf"
    )]
    model: String,

    /// Q4 GGUF model file (use instead of --model for quantized inference).
    #[arg(long, conflicts_with = "model")]
    gguf: Option<String>,

    /// Voice preset name.
    #[arg(short, long, default_value = "casual_female")]
    voice: String,

    /// Voice embeddings directory (auto-discovered from --model dir for BF16).
    #[arg(long)]
    voices_dir: Option<String>,

    /// Output WAV file path.
    #[arg(short, long, default_value = "output.wav")]
    output: String,

    /// Tekken tokenizer JSON (auto-discovered from model dir).
    #[arg(long)]
    tokenizer: Option<String>,

    /// List available voice presets and exit.
    #[arg(long)]
    list_voices: bool,

    /// Maximum audio frames to generate.
    #[arg(long, default_value_t = 2000)]
    max_frames: usize,

    /// Euler ODE steps: 3=real-time, 4=balanced, 8=quality.
    #[arg(long, default_value_t = 4)]
    euler_steps: usize,
}

pub fn process_text(text: &String) -> Result<()> {
    let txt = &text.to_string();
    let sentences = segment("fr", txt);
    for sentence in &sentences {
        EKO!(sentence);
        EKO!(words_count::count(sentence).words);
    }
    Ok(())    
}

pub fn run(args: Args) -> Result<()> {
    let device = burn::backend::wgpu::WgpuDevice::default();
    let j = 1234;
    let k = "abc";
    EKO!(j);
    EKO!(j, k);

    //use tch::{nn, Device, Tensor, Kind};

    //tch::manual_seed(42); // Fixed seed for reproducibility

    
    if let Some(txt1) = &args.text {    
        let _ = process_text(&txt1);
    }

    // Resolve tokenizer
    let tokenizer_path = match &args.tokenizer {
        Some(p) => PathBuf::from(p),
        None => {
            if let Some(gguf) = &args.gguf {
                // Try alongside GGUF, then common locations
                let gguf_dir = PathBuf::from(gguf)
                    .parent()
                    .unwrap_or(&PathBuf::from("."))
                    .to_path_buf();
                let candidates = [
                    gguf_dir.join("tekken.json"),
                    PathBuf::from("models/voxtral-tts/tekken.json"),
                    PathBuf::from("models/voxtral/tekken.json"),
                ];
                candidates
                    .into_iter()
                    .find(|p| p.exists())
                    .ok_or_else(|| anyhow::anyhow!("Tokenizer not found. Provide --tokenizer"))?
            } else {
                PathBuf::from(&args.model).join("tekken.json")
            }
        }
    };


    
    EKO!(&tokenizer_path);
    // Get token IDs
    let (token_ids, stns) : (Vec<Vec<u32>>, Vec<String>) = if let Some(ids) = &args.token_ids {
        
        (vec![ids.clone()], vec!["tokens".to_string()])
    } else if let Some(text) = &args.text {
        if !tokenizer_path.exists() {
            bail!("Tokenizer not found at {}", tokenizer_path.display());
        }
        let encoder =
            TekkenEncoder::from_file(&tokenizer_path).context("Failed to load tokenizer")?;
        EKO!("encoding");
        (vec![encoder.encode(text)], vec![text.to_string()])
    }  else if let Some(file_text) = &args.file_text {
        if !tokenizer_path.exists() {
            bail!("Tokenizer not found at {}", tokenizer_path.display());
        }
        EKO!(file_text);

        let contents1 = fs::read_to_string(file_text).expect("Should have been able to read the file");
        let contents2 = contents1.trim().replace("\n", " ");

        let contents66 = contents1
            .split('\n')
            .filter(|short| !short.is_empty())
            .collect::<Vec<_>>();
        

        
        let contents3 = contents2
        .replace(['\n', '\t'], " ")
        .trim()
        .split(' ')
        .filter(|short| !short.is_empty())
        .collect::<Vec<_>>()
        .join(" ");
        let contents = contents3.split_whitespace().collect::<Vec<_>>().join(" ");
        EKO!(&contents);
        let encoder =
            TekkenEncoder::from_file(&tokenizer_path).context("Failed to load tokenizer")?;
        EKO!("encoding");
        let mut v: Vec<Vec<u32>> = vec![];        
        let mut vs: Vec<String> = vec![];        
        let mut sentences = segment("fr", contents.as_str());

        sentences = contents66;
        
        for sentence in &sentences {
            //EKO!(sentence);
            //EKO!(words_count::count(sentence).words);
            let mut ss = vec![sentence.to_string()];
            while ss.len() > 0 {
                let s0 = ss.remove(0);
                let ens = encoder.encode(&s0);
                //EKO!(ens.len());
                let length = s0.len();
                
                if ens.len() > 4000 {
                    let mut ccc = length/2;
                    let space = " ".chars().next();
                    while s0.chars().nth(ccc) != space {
                        ccc += 1;
                    }
                    let s1 = s0[0..ccc].to_string();
                    let s2 = s0[ccc..].to_string();                    
                    ss.push(s1);
                    ss.push(s2);
                } else {
                    v.push(ens);
                    vs.push(s0.to_string());
                }
            }
        }
        (v, vs)
    } else if !args.list_voices {
        bail!("--text or --token-ids required");
    } else {
        (vec![], vec![])
    };
    EKO!();
    if let Some(gguf_path) = &args.gguf {
        EKO!();
        let vv = token_ids;
        let _rr = run_q4_l(&args, gguf_path, vv, stns, &device);
        Ok(())
            
    } else {
        run_bf16(&args, &token_ids, &device)
    }
}

/// BF16 SafeTensors path.
fn run_bf16(
    args: &Args,
    token_ids: &Vec<Vec<u32>>,
    device: &<Backend as burn::tensor::backend::Backend>::Device,
) -> Result<()> {
    use voxtral_mini_realtime::tts::pipeline::TtsPipeline;

    let model_dir = PathBuf::from(&args.model);
    if !model_dir.join("consolidated.safetensors").exists() {
        bail!(
            "Model not found at {}\nDownload: hf download mistralai/Voxtral-4B-TTS-2603 --local-dir {}",
            model_dir.join("consolidated.safetensors").display(),
            model_dir.display()
        );
    }

    let start = Instant::now();
    EKO!(start);
    info!("Loading BF16 TTS pipeline from {}", model_dir.display());
    let mut pipeline =
        TtsPipeline::<Backend>::from_model_dir(&model_dir, device).context("Failed to load")?;
    pipeline.set_euler_steps(args.euler_steps);
    EKO!();
    info!(
        elapsed_ms = start.elapsed().as_millis() as u64,
        euler_steps = args.euler_steps,
        "BF16 pipeline loaded"
    );

    if args.list_voices {
        let voices = pipeline.list_voices();
        println!("Available voices ({}):", voices.len());
        for name in &voices {
            println!("  {name}");
        }
        return Ok(());
    }

    if !pipeline.has_voice(&args.voice) {
        bail!(
            "Voice '{}' not found. Use --list-voices to see available presets",
            args.voice
        );
    }

    info!(
        text_tokens = token_ids.len(),
        voice = %args.voice,
        "Synthesizing"
    );
    let gen_start = Instant::now();
    EKO!(gen_start);
    let audio =
        pipeline.generate_with_max_frames(token_ids[0].as_slice(), &args.voice, args.max_frames, device)?;
    let duration = audio.len() as f64 / audio.sample_rate as f64;
    info!(
        elapsed_ms = gen_start.elapsed().as_millis() as u64,
        duration_sec = format!("{duration:.2}"),
        "Audio generated"
    );

    save_audio(&audio, &args.output, gen_start.elapsed(), duration)
}

/// Q4 GGUF path.
fn run_q4_l(
    args: &Args,
    gguf_path: &str,
    token_ids_list: Vec<Vec<u32>>,
    stns : Vec<String>,
    device: &burn::backend::wgpu::WgpuDevice,
) -> Result<() > {
    //use voxtral_mini_realtime::gguf::Q4TtsModelLoader;
    use voxtral_mini_realtime::tts::config::{AudioCodebookLayout, TtsSpecialTokens};
    use voxtral_mini_realtime::tts::embeddings::AudioCodebookEmbeddings;
    use voxtral_mini_realtime::tts::voice::load_voice_from_bytes;
    EKO!();
    
    EKO!("start");
    let time_format = "%I:%M:%S %p";
    //let mut time_now = String::new();
    
    let time_now = chrono::Local::now()
        .format(time_format)
        .to_string();
    EKO!(time_now);
    EKO!("loading model");

    
    let path = PathBuf::from(gguf_path);

    if !path.exists() {
        bail!("GGUF model not found at {}", path.display());
    }

    EKO!();
    let start = Instant::now();
    info!("Loading Q4 TTS model from {}", path.display());
    let mut loader = Q4TtsModelLoader::from_file(&path).context("Failed to open GGUF")?;
    EKO!(start);
    let (backbone, mut fm, codec) = loader.load(device).context("Failed to load Q4 model")?;
    fm.set_euler_steps(args.euler_steps);
    EKO!();
    info!(
        elapsed_ms = start.elapsed().as_millis() as u64,
        euler_steps = args.euler_steps,
        "Q4 model loaded"
    );
    EKO!(start.elapsed().as_millis() as u64);
    EKO!("loading voices");
    // Resolve voices directory
    let voices_dir = match &args.voices_dir {
        Some(d) => PathBuf::from(d),
        None => PathBuf::from("models/voxtral-tts/voice_embedding"),
    };
    EKO!(&voices_dir);
    if args.list_voices {
        /*
        if !voices_dir.exists() {
            bail!("Voices directory not found at {}", voices_dir.display());
        }*/
        let mut voices: Vec<String> = std::fs::read_dir(&voices_dir)?
            .filter_map(|e| e.ok())
            .filter(|e| e.path().extension().is_some_and(|ext| ext == "safetensors"))
            .filter_map(|e| {
                e.path()
                    .file_stem()
                    .map(|s| s.to_string_lossy().into_owned())
            })
            .collect();
        voices.sort();
        EKO!(voices.len());
        println!("Available voices ({}):", voices.len());
        for name in &voices {
            println!("  {name}");
        }
        return Ok(());
    }

    // Load voice
    let voice_path = voices_dir.join(format!("{}.safetensors", args.voice));
    if !voice_path.exists() {
        EKO!(&voice_path);
        bail!(
            "Voice '{}' not found at {}\nUse --list-voices to see available presets",
            args.voice,
            voice_path.display()
        );
    }
    EKO!(&voice_path);
    let voice_bytes = std::fs::read(&voice_path)?;
    let voice_embed: Tensor<Backend, 2> =
        load_voice_from_bytes(&voice_bytes, 3072, device).context("Failed to load voice")?;
    info!(
        voice = %args.voice,
        frames = voice_embed.dims()[0],
        "Voice loaded"
    );

    EKO!(voice_embed.shape());
    
    EKO!("embedding text");
    // Build input sequence
    let special = TtsSpecialTokens::default();
    let bos = backbone.embed_tokens_from_ids(&[special.bos_token_id as i32], 1, 1);
    let begin_audio = backbone.embed_tokens_from_ids(&[special.begin_audio_token_id as i32], 1, 1);
    let next_audio_text =
        backbone.embed_tokens_from_ids(&[special.next_audio_text_token_id as i32], 1, 1);
    let repeat_audio_text =
        backbone.embed_tokens_from_ids(&[special.repeat_audio_text_token_id as i32], 1, 1);

    let closure = |token_ids: &Vec<u32>, sss : String, _index_ : u32| -> AudioBuffer {
        println!("calculating slowly...");
        let samples : Vec<f32>  = vec![1.2];
        let _audio = AudioBuffer::new(samples, 1);
        EKO!(_index_);
        let gen_start = Instant::now();        
        let do_save = 1>3;
        
        let text_ids_i32: Vec<i32> = token_ids.iter().map(|&id| id as i32).collect();
        let text_embeds = backbone.embed_tokens_from_ids(&text_ids_i32, 1, text_ids_i32.len());
        EKO!(&text_embeds.shape());
        EKO!(&sss);
        let input_sequence = Tensor::cat(
            vec![
                bos.clone(),
                begin_audio.clone(),
                voice_embed.clone().unsqueeze_dim::<3>(0),
                next_audio_text.clone(),
                text_embeds,
                repeat_audio_text.clone(),
                begin_audio.clone(),
            ],
            1,
        );
        EKO!(input_sequence.shape());
        EKO!(token_ids.len());
        let codebook = AudioCodebookEmbeddings::new(
            backbone.audio_codebook_embeddings().clone(),
            AudioCodebookLayout::default(),
        );
        
        info!(
            text_tokens = token_ids.len(),
            voice = %args.voice,
            "Synthesizing"
        );
        //println!("generate");
        EKO!("generating");

        EKO!(gen_start.elapsed().as_millis() as u64);
        let generate_start = Instant::now();                
        let frames = pollster::block_on(backbone.generate_async(
            input_sequence,
            &fm,
            &codebook,
            args.max_frames,
        ))
            .map_err(|e| anyhow::anyhow!("Generation failed: {e}")).unwrap();
        
        /*
        if frames.is_empty() {
            bail!("No audio frames generated");
        }
         */

        let generatione_ms = generate_start.elapsed().as_millis() as f32;
        EKO!(generatione_ms);
        EKO!(generatione_ms  / (token_ids.len() as f32));
        
        // Codec decode
        let n_frames = frames.len();
        EKO!(generatione_ms / (n_frames as f32));
        EKO!(n_frames);
        println!("codec {}", n_frames);
        let semantic_indices: Vec<usize> = frames.iter().map(|f| f.semantic_idx).collect();
        let mut acoustic_data = Vec::with_capacity(n_frames * 36);
        for frame in &frames {
            for &level in &frame.acoustic_levels {
                acoustic_data.push(level as f32);
            }
        }
        let acoustic_tensor: Tensor<Backend, 2> = Tensor::from_data(
            burn::tensor::TensorData::new(acoustic_data, [n_frames, 36]),
            device,
        );
        EKO!(acoustic_tensor.shape());
        EKO!(semantic_indices.len());
        EKO!(generatione_ms / (semantic_indices.len() as f32));
        EKO!(&args.output);
        EKO!("decoding semantic token into audio");
        
        if 1>2 {
            let block_size = 800;
            let mut start = 0;
            while 1>0 {
                EKO!(start);
                if start >=  semantic_indices.len() {
                    break;
                }            
                let end = std::cmp::min(start + block_size, semantic_indices.len());
                let si = &semantic_indices[start .. end];
                let acoustic_tensor1 = acoustic_tensor.clone().slice(s![start..end, ..]);
                start = end ;
                
                EKO!(acoustic_tensor1.shape());
                EKO!(si.len());
                EKO!("decodng 1");
                let _waveform =  codec.decode(&si, acoustic_tensor1);
                EKO!();
            }
        }

        let decoding_start = Instant::now();
        
        EKO!("decding 2");


        let closure2 = |nn : u32| -> AudioBuffer {
            
            let waveform =  codec.decode(&semantic_indices, acoustic_tensor);
            let [_batch, total_samples] = waveform.dims();
            EKO!(total_samples);
            
            let decoding_ms = decoding_start.elapsed().as_millis() as u64;
            EKO!(decoding_ms);
            EKO!(decoding_ms / (total_samples as u64) );
            
            EKO!("saving audio");
            let wav_data = waveform.to_data();
            let mut samples: Vec<f32> = wav_data.as_slice::<f32>().unwrap()[..total_samples].to_vec();
            EKO!(samples.len());
            let peak = samples.iter().map(|s| s.abs()).fold(0.0f32, f32::max);
            if peak > 1e-6 {
                let gain = 0.95 / peak;
                EKO!(gain);
                for s in &mut samples {
                    *s *= gain;
                }
            }
            EKO!();
            let gain_one_mn = 8.;
            for (index, value) in samples.iter_mut().enumerate() {
                let gi = 1. + (gain_one_mn - 1.)/1000000. * index as f32;
                *value = *value * gi;
                //println!("Index: {}, Value: {}", index, value);
            }
            
            let audio = AudioBuffer::new(samples, 24000);
            EKO!(audio.len());
            let duration = audio.len() as f64 / audio.sample_rate as f64;
            info!(
                elapsed_ms = gen_start.elapsed().as_millis() as u64,
                frames = n_frames,
                duration_sec = format!("{duration:.2}"),
                "Audio generated"
            );
            
            let ext = Path::new(&args.output).extension().unwrap().to_str().unwrap();
            let stem = Path::new(&args.output).file_stem().unwrap().to_str().unwrap();
            EKO!(stem.to_owned() + ext);
            let sindex: String = format!("{:0>5}", _index_.to_string());        
            let ss = stem.to_owned().to_string() + &sindex + "." + ext; //.to_string();
            EKO!(do_save);
            if do_save {
                let _ = save_audio(&audio, &ss.to_string(), gen_start.elapsed(), duration);
            }
            audio
        };
        closure2(1)
    };
    let gen_start = Instant::now();
    let samples : Vec<f32>  = vec![0.0];
    let mut res = AudioBuffer::new(samples, 24000);
    EKO!(token_ids_list.len());
    for (iel, (tki, sss)) in zip(token_ids_list, stns).enumerate() {
        EKO!();
        let _ab = closure(&tki, sss, iel as u32);
        let _ = res.append(&_ab);
        EKO!(res.len());
    }
    let duration = res.len() as f64 / res.sample_rate as f64;
    let _ = save_audio(&res, &args.output, gen_start.elapsed(), duration);
    //let _ = token_ids_list.iter().map(|tki| run_q4(args, gguf_path, tki, device));
    //EKO!(&rr);
    Ok(())
}

fn save_audio(
    audio: &AudioBuffer,
    output: &str,
    gen_elapsed: std::time::Duration,
    duration: f64,
) -> Result<()> {
    let output_path = PathBuf::from(output);
    audio
        .save(&output_path)
        .with_context(|| format!("Failed to save {}", output_path.display()))?;

    let rtf = gen_elapsed.as_secs_f64() / duration;
    println!(
        "Saved {:.2}s of audio to {} (RTF {:.2}x)",
        duration,
        output_path.display(),
        rtf,
    );
    Ok(())
}
