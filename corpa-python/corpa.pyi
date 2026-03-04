from typing import List, Optional, TypedDict

class StatsResult(TypedDict):
    tokens: int
    types: int
    characters: int
    sentences: int
    type_token_ratio: float
    hapax_legomena: int
    hapax_percentage: float
    avg_sentence_length: float
    stopwords_removed: Optional[int]

class NgramEntry(TypedDict):
    ngram: str
    frequency: int
    relative_pct: float

class EntropyResult(TypedDict):
    h1: float
    h2: float
    h3: float
    entropy_rate: float
    vocabulary_size: int
    redundancy: float

class ReadabilityResult(TypedDict):
    flesch_kincaid_grade: float
    flesch_reading_ease: float
    coleman_liau: float
    gunning_fog: float
    smog: float

class PerplexityResult(TypedDict):
    order: int
    vocab_size: int
    ngram_counts: List[int]
    smoothing: str
    perplexity: float

class LangResult(TypedDict):
    language: str
    code: str
    script: str
    confidence: float
    is_reliable: bool

class TokensResult(TypedDict):
    whitespace: int
    sentences: int
    characters: int
    bpe_gpt4: Optional[int]
    bpe_gpt4o: Optional[int]
    bpe_gpt3: Optional[int]

class ZipfEntry(TypedDict):
    rank: int
    word: str
    frequency: int

class ZipfResult(TypedDict):
    entries: List[ZipfEntry]
    alpha: float
    r_squared: float

def stats(
    path: Optional[str] = None,
    *,
    text: Optional[str] = None,
    stopwords: Optional[List[str]] = None,
) -> StatsResult: ...

def ngrams(
    path: Optional[str] = None,
    *,
    text: Optional[str] = None,
    n: int = 2,
    top: int = 10,
    min_freq: Optional[int] = None,
    case_insensitive: bool = False,
    stopwords: Optional[List[str]] = None,
) -> List[NgramEntry]: ...

def entropy(
    path: Optional[str] = None,
    *,
    text: Optional[str] = None,
) -> EntropyResult: ...

def readability(
    path: Optional[str] = None,
    *,
    text: Optional[str] = None,
) -> ReadabilityResult: ...

def perplexity(
    path: Optional[str] = None,
    *,
    text: Optional[str] = None,
    order: int = 3,
    smoothing: str = "laplace",
    k: float = 1.0,
) -> PerplexityResult: ...

def lang(
    path: Optional[str] = None,
    *,
    text: Optional[str] = None,
) -> Optional[LangResult]: ...

def tokens(
    path: Optional[str] = None,
    *,
    text: Optional[str] = None,
    include_bpe: bool = True,
) -> TokensResult: ...

def zipf(
    path: Optional[str] = None,
    *,
    text: Optional[str] = None,
    top: int = 20,
) -> ZipfResult: ...
