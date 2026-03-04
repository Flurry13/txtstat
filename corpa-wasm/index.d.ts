export interface StatsResult {
  tokens: number;
  types: number;
  characters: number;
  sentences: number;
  type_token_ratio: number;
  hapax_legomena: number;
  hapax_percentage: number;
  avg_sentence_length: number;
  stopwords_removed: number | null;
}

export interface NgramEntry {
  ngram: string;
  frequency: number;
  relative_pct: number;
}

export interface EntropyResult {
  h1: number;
  h2: number;
  h3: number;
  entropy_rate: number;
  vocabulary_size: number;
  redundancy: number;
}

export interface ReadabilityResult {
  flesch_kincaid_grade: number;
  flesch_reading_ease: number;
  coleman_liau: number;
  gunning_fog: number;
  smog: number;
}

export interface PerplexityResult {
  order: number;
  vocab_size: number;
  ngram_counts: number[];
  smoothing: string;
  perplexity: number;
}

export interface LangResult {
  language: string;
  code: string;
  script: string;
  confidence: number;
  is_reliable: boolean;
}

export interface TokensResult {
  whitespace: number;
  sentences: number;
  characters: number;
  bpe_gpt4: number | null;
  bpe_gpt4o: number | null;
  bpe_gpt3: number | null;
}

export interface ZipfEntry {
  rank: number;
  word: string;
  frequency: number;
}

export interface ZipfResult {
  entries: ZipfEntry[];
  alpha: number;
  r_squared: number;
}

export function stats(text: string, stopwords?: string[] | null): StatsResult;
export function ngrams(
  text: string,
  n: number,
  top: number,
  min_freq?: number | null,
  case_insensitive?: boolean | null,
  stopwords?: string[] | null,
): NgramEntry[];
export function entropy(text: string): EntropyResult;
export function readability(text: string): ReadabilityResult;
export function lang(text: string): LangResult | null;
export function perplexity(
  text: string,
  order: number,
  smoothing: string,
  k: number,
): PerplexityResult;
export function zipf(text: string, top: number): ZipfResult;
export function tokens(text: string): TokensResult;
