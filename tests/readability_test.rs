use lexis::analysis::readability;

#[test]
fn test_compute_metrics_basic() {
    let text = "The cat sat on the mat. The dog ran fast.";
    let m = readability::compute_metrics(text);
    assert_eq!(m.word_count, 10);
    assert_eq!(m.sentence_count, 2);
    assert!(m.syllable_count > 0);
    assert!(m.char_count > 0);
}

#[test]
fn test_flesch_kincaid_grade() {
    let m = readability::TextMetrics {
        word_count: 100,
        sentence_count: 5,
        syllable_count: 150,
        char_count: 400,
        complex_word_count: 10,
    };
    let grade = readability::flesch_kincaid_grade(&m);
    // 0.39*(100/5) + 11.8*(150/100) - 15.59 = 7.8 + 17.7 - 15.59 = 9.91
    assert!((grade - 9.91).abs() < 0.1, "got {}", grade);
}

#[test]
fn test_flesch_reading_ease() {
    let m = readability::TextMetrics {
        word_count: 100,
        sentence_count: 5,
        syllable_count: 150,
        char_count: 400,
        complex_word_count: 10,
    };
    let ease = readability::flesch_reading_ease(&m);
    // 206.835 - 1.015*20 - 84.6*1.5 = 206.835 - 20.3 - 126.9 = 59.635
    assert!((ease - 59.635).abs() < 0.1, "got {}", ease);
}

#[test]
fn test_coleman_liau() {
    let m = readability::TextMetrics {
        word_count: 100,
        sentence_count: 5,
        syllable_count: 150,
        char_count: 400,
        complex_word_count: 10,
    };
    let cl = readability::coleman_liau(&m);
    // 0.0588*400 - 0.296*5 - 15.8 = 23.52 - 1.48 - 15.8 = 6.24
    assert!((cl - 6.24).abs() < 0.1, "got {}", cl);
}

#[test]
fn test_gunning_fog() {
    let m = readability::TextMetrics {
        word_count: 100,
        sentence_count: 5,
        syllable_count: 150,
        char_count: 400,
        complex_word_count: 10,
    };
    let fog = readability::gunning_fog(&m);
    // 0.4*(20 + 10) = 12.0
    assert!((fog - 12.0).abs() < 0.1, "got {}", fog);
}

#[test]
fn test_smog() {
    let m = readability::TextMetrics {
        word_count: 100,
        sentence_count: 5,
        syllable_count: 150,
        char_count: 400,
        complex_word_count: 10,
    };
    let smog = readability::smog(&m);
    // 3 + sqrt(10*30/5) = 3 + sqrt(60) = 3 + 7.746 = 10.746
    assert!((smog - 10.746).abs() < 0.1, "got {}", smog);
}

#[test]
fn test_empty_text() {
    let m = readability::compute_metrics("");
    assert_eq!(m.word_count, 0);
    assert_eq!(readability::flesch_kincaid_grade(&m), 0.0);
    assert_eq!(readability::flesch_reading_ease(&m), 0.0);
    assert_eq!(readability::coleman_liau(&m), 0.0);
    assert_eq!(readability::gunning_fog(&m), 0.0);
    assert_eq!(readability::smog(&m), 0.0);
}

#[test]
fn test_grade_label_boundaries() {
    assert_eq!(readability::grade_label(3.0), "Elementary");
    assert_eq!(readability::grade_label(7.0), "Middle School");
    assert_eq!(readability::grade_label(10.0), "High School");
    assert_eq!(readability::grade_label(14.0), "College");
    assert_eq!(readability::grade_label(18.0), "Graduate");
}

#[test]
fn test_ease_label_boundaries() {
    assert_eq!(readability::ease_label(95.0), "Very Easy");
    assert_eq!(readability::ease_label(85.0), "Easy");
    assert_eq!(readability::ease_label(75.0), "Fairly Easy");
    assert_eq!(readability::ease_label(65.0), "Standard");
    assert_eq!(readability::ease_label(55.0), "Fairly Difficult");
    assert_eq!(readability::ease_label(40.0), "Difficult");
    assert_eq!(readability::ease_label(20.0), "Very Difficult");
}
