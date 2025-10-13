#!/bin/bash
# Download arXiv papers for benchmark testing

OUTPUT_DIR="data/papers"
mkdir -p "$OUTPUT_DIR"

echo "📥 Downloading arXiv papers for benchmark..."
echo "Output directory: $OUTPUT_DIR"
echo ""

# Function to download paper
download_paper() {
    local arxiv_id=$1
    local name=$2
    local url="https://arxiv.org/pdf/${arxiv_id}.pdf"
    local output_file="${OUTPUT_DIR}/${arxiv_id}_${name}.pdf"
    
    if [ -f "$output_file" ]; then
        echo "  ✓ Already exists: $name"
        return 0
    fi
    
    echo "  📄 Downloading: $name ($arxiv_id)..."
    wget -q "$url" -O "$output_file" 2>&1
    
    if [ $? -eq 0 ]; then
        local size=$(du -h "$output_file" | cut -f1)
        echo "     ✓ Downloaded: $size"
        return 0
    else
        echo "     ✗ Failed to download $arxiv_id"
        rm -f "$output_file"
        return 1
    fi
}

echo "═══════════════════════════════════════════════════════"
echo "FOUNDATIONAL MODELS"
echo "═══════════════════════════════════════════════════════"

download_paper "1706.03762" "attention"
download_paper "1801.06146" "ulmfit"
download_paper "1802.05365" "elmo"
download_paper "1810.04805" "bert"
download_paper "1909.08053" "megatron"
download_paper "1907.11692" "roberta"
download_paper "1906.08237" "xlnet"
download_paper "1909.11942" "albert"
download_paper "1910.10683" "t5"

echo ""
echo "═══════════════════════════════════════════════════════"
echo "SCALE, EFFICIENCY & ARCHITECTURES"
echo "═══════════════════════════════════════════════════════"

download_paper "2005.14165" "gpt3"
download_paper "2004.05150" "longformer"
download_paper "2001.04451" "reformer"
download_paper "2007.14062" "bigbird"
download_paper "2101.03961" "switch_transformer"
download_paper "2006.16668" "gshard"
download_paper "2203.15556" "chinchilla"
download_paper "2204.02311" "palm"
download_paper "2205.05131" "ul2"
download_paper "2302.13971" "llama"
download_paper "2307.09288" "llama2"
download_paper "2310.06825" "mistral"
download_paper "2401.04088" "mixtral"
download_paper "2312.00752" "mamba"
download_paper "2302.10866" "hyena"

echo ""
echo "═══════════════════════════════════════════════════════"
echo "INSTRUCTION, ALIGNMENT & PREFERENCE"
echo "═══════════════════════════════════════════════════════"

download_paper "2203.02155" "instructgpt"
download_paper "2212.08073" "constitutional_ai"
download_paper "2305.18290" "dpo"
download_paper "2203.11171" "self_consistency"
download_paper "2201.11903" "chain_of_thought"
download_paper "2210.11416" "flan_t5"
download_paper "2302.04761" "toolformer"
download_paper "2303.11366" "reflexion"
download_paper "2212.10560" "self_instruct"
download_paper "2309.00267" "rlaif"

echo ""
echo "═══════════════════════════════════════════════════════"
echo "LIGHTWEIGHT TRAINING & PEFT"
echo "═══════════════════════════════════════════════════════"

download_paper "2106.09685" "lora"
download_paper "2305.14314" "qlora"
download_paper "2208.07339" "llm_int8"
download_paper "2211.10438" "smoothquant"
download_paper "2210.17323" "gptq"
download_paper "2306.00978" "awq"
download_paper "2005.00247" "adapter_fusion"
download_paper "2304.15010" "llama_adapter"
download_paper "2303.15647" "peft_survey"
download_paper "2406.16564" "lora_survey"

echo ""
echo "═══════════════════════════════════════════════════════"
echo "RETRIEVAL, RAG & INDEXING"
echo "═══════════════════════════════════════════════════════"

download_paper "2004.04906" "dpr"
download_paper "2005.11401" "rag"
download_paper "2110.11386" "colbert_v2"
download_paper "2112.09118" "contriever"
download_paper "2302.10924" "e5_embeddings"
download_paper "2307.02179" "bge_embeddings"
download_paper "2307.16877" "radit"
download_paper "2310.11511" "self_rag"
download_paper "2305.09612" "fresh_llms"
download_paper "2401.00544" "rag_survey"

echo ""
echo "═══════════════════════════════════════════════════════"
echo "MULTIMODAL (VISION-LANGUAGE, AUDIO, VIDEO)"
echo "═══════════════════════════════════════════════════════"

download_paper "2103.00020" "clip"
download_paper "2102.12092" "dalle"
download_paper "2301.12597" "blip2"
download_paper "2204.14198" "flamingo"
download_paper "2302.14045" "kosmos1"
download_paper "2304.08485" "llava"
download_paper "2305.18565" "palix"
download_paper "2308.12966" "qwen_vl"
download_paper "2212.04356" "whisper"
download_paper "2306.02858" "video_llama"

echo ""
echo "═══════════════════════════════════════════════════════"
echo "DIFFUSION & GENERATION"
echo "═══════════════════════════════════════════════════════"

download_paper "2006.11239" "ddpm"
download_paper "2105.05233" "guided_diffusion"
download_paper "2112.10752" "stable_diffusion"
download_paper "2205.11487" "imagen"
download_paper "2206.10789" "parti"
download_paper "2302.05543" "controlnet"
download_paper "2209.00796" "diffusion_survey"
download_paper "2303.01469" "consistency_models"
download_paper "2209.03003" "rectified_flow"
download_paper "2204.03458" "video_diffusion"

echo ""
echo "═══════════════════════════════════════════════════════"
echo "AGENTS, TOOLS & REASONING"
echo "═══════════════════════════════════════════════════════"

download_paper "2210.03629" "react"
download_paper "2305.15334" "gorilla"
download_paper "2305.16291" "voyager"
download_paper "2305.10601" "tree_of_thought"
download_paper "2308.09687" "graph_of_thoughts"
download_paper "2211.10435" "pal"
download_paper "2308.00352" "autogpt_survey"
download_paper "2308.03688" "agentbench"

echo ""
echo "═══════════════════════════════════════════════════════"
echo "INFERENCE & TRAINING OPTIMIZATIONS"
echo "═══════════════════════════════════════════════════════"

download_paper "2205.14135" "flash_attention"
download_paper "2307.08691" "flash_attention2"
download_paper "2302.01318" "speculative_decoding"
download_paper "2307.06147" "medusa"
download_paper "2309.17453" "streaming_llm"
download_paper "2306.15595" "position_interpolation"
download_paper "2309.00071" "yarn"
download_paper "2306.14882" "longrope"
download_paper "1909.05025" "local_attention"
download_paper "2301.03372" "token_merging"

echo ""
echo "═══════════════════════════════════════════════════════"
echo "EVALUATION, BENCHMARKING & SAFETY"
echo "═══════════════════════════════════════════════════════"

download_paper "2009.03300" "mmlu"
download_paper "2206.04615" "bigbench"
download_paper "2211.09110" "helm"
download_paper "2309.07864" "red_teaming"
download_paper "2308.05374" "safety_survey"

echo ""
echo "════════════════════════════════════════════════════════"
echo "✅ DOWNLOAD COMPLETE!"
echo "════════════════════════════════════════════════════════"
echo ""
echo "📊 Statistics:"
total_count=$(ls -1 "$OUTPUT_DIR"/*.pdf 2>/dev/null | wc -l)
total_size=$(du -sh "$OUTPUT_DIR" | cut -f1)
echo "  Papers downloaded: $total_count"
echo "  Total size: $total_size"
echo "  Location: $OUTPUT_DIR/"
echo ""

