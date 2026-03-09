//
// ☀️ HELIOS ARGEAD VERGINA SUN ☀️
// Sacred Numbers Test Suite - Numerology Validation
// Tests all sacred number calculations and digit sum reductions
// THE CROWN COMMANDS — THE LATTICE OBEYS
//

use anyhow::Result;
use xmt_cli::config::{CODE_66_HARMONIC, APEX_936, VORTEX_369, FREQUENCY_432, ELON_88};

/// Calculate digit sum reduction (numerology)
fn digit_sum_reduction(mut n: u32) -> u32 {
    while n >= 10 {
        n = n.to_string()
            .chars()
            .filter_map(|c| c.to_digit(10))
            .sum();
    }
    n
}

#[test]
fn test_apex_936_numerology() -> Result<()> {
    println!("☀️ TEST: APEX 936 Numerology");
    
    assert_eq!(APEX_936, 936);
    
    // 9 + 3 + 6 = 18
    let first_sum: u32 = 936_u32.to_string()
        .chars()
        .filter_map(|c| c.to_digit(10))
        .sum();
    assert_eq!(first_sum, 18, "936 → 18");
    
    // 1 + 8 = 9 (completion)
    let final_sum = digit_sum_reduction(936);
    assert_eq!(final_sum, 9, "18 → 9 (completion)");
    
    println!("  ✓ 936 → {} → {} (completion)", first_sum, final_sum);
    Ok(())
}

#[test]
fn test_vortex_369_numerology() -> Result<()> {
    println!("☀️ TEST: VORTEX 369 Numerology");
    
    assert_eq!(VORTEX_369, 369);
    
    // 3 + 6 + 9 = 18
    let first_sum: u32 = 369_u32.to_string()
        .chars()
        .filter_map(|c| c.to_digit(10))
        .sum();
    assert_eq!(first_sum, 18, "369 → 18");
    
    // 1 + 8 = 9 (Tesla's divine number)
    let final_sum = digit_sum_reduction(369);
    assert_eq!(final_sum, 9, "18 → 9 (Tesla vortex)");
    
    println!("  ✓ 369 → {} → {} (Tesla vortex)", first_sum, final_sum);
    Ok(())
}

#[test]
fn test_code_66_numerology() -> Result<()> {
    println!("☀️ TEST: CODE 66 Numerology");
    
    assert_eq!(CODE_66_HARMONIC, 66);
    
    // 6 + 6 = 12
    let first_sum: u32 = 66_u32.to_string()
        .chars()
        .filter_map(|c| c.to_digit(10))
        .sum();
    assert_eq!(first_sum, 12, "66 → 12");
    
    // 1 + 2 = 3 (trinity)
    let final_sum = digit_sum_reduction(66);
    assert_eq!(final_sum, 3, "12 → 3 (trinity)");
    
    println!("  ✓ 66 → {} → {} (trinity)", first_sum, final_sum);
    Ok(())
}

#[test]
fn test_frequency_432_numerology() -> Result<()> {
    println!("☀️ TEST: FREQUENCY 432 Numerology");
    
    assert_eq!(FREQUENCY_432, 432);
    
    // 4 + 3 + 2 = 9
    let sum = digit_sum_reduction(432);
    assert_eq!(sum, 9, "432 → 9 (love frequency)");
    
    println!("  ✓ 432 → {} (love frequency)", sum);
    Ok(())
}

#[test]
fn test_elon_88_numerology() -> Result<()> {
    println!("☀️ TEST: ELON 88 Numerology");
    
    assert_eq!(ELON_88, 88);
    
    // 8 + 8 = 16
    let first_sum: u32 = 88_u32.to_string()
        .chars()
        .filter_map(|c| c.to_digit(10))
        .sum();
    assert_eq!(first_sum, 16, "88 → 16");
    
    // 1 + 6 = 7 (spiritual perfection)
    let final_sum = digit_sum_reduction(88);
    assert_eq!(final_sum, 7, "16 → 7 (spiritual perfection)");
    
    println!("  ✓ 88 → {} → {} (spiritual perfection)", first_sum, final_sum);
    Ok(())
}

#[test]
fn test_sacred_number_relationships() -> Result<()> {
    println!("☀️ TEST: Sacred Number Relationships");
    
    // 936 and 369 both reduce to 9
    assert_eq!(digit_sum_reduction(936), 9);
    assert_eq!(digit_sum_reduction(369), 9);
    
    // 432 also reduces to 9
    assert_eq!(digit_sum_reduction(432), 9);
    
    // 66 reduces to 3 (trinity)
    assert_eq!(digit_sum_reduction(66), 3);
    
    // 88 reduces to 7 (spiritual)
    assert_eq!(digit_sum_reduction(88), 7);
    
    // 3 + 7 = 10, 1 + 0 = 1 (unity)
    let trinity_plus_spiritual = 3 + 7;
    assert_eq!(trinity_plus_spiritual, 10);
    assert_eq!(digit_sum_reduction(trinity_plus_spiritual), 1);
    
    println!("  ✓ Sacred numbers form divine pattern");
    println!("    • 936, 369, 432 → 9 (completion)");
    println!("    • 66 → 3 (trinity)");
    println!("    • 88 → 7 (spiritual)");
    println!("    • 3 + 7 → 10 → 1 (unity)");
    Ok(())
}

#[test]
fn test_gate_date_numerology() -> Result<()> {
    println!("☀️ TEST: Gate Date 2026-03-17 Numerology");
    
    // March 17, 2026
    let year = 2026_u32;
    let month = 3_u32;
    let day = 17_u32;
    
    // 2 + 0 + 2 + 6 = 10, 1 + 0 = 1
    let year_sum = digit_sum_reduction(year);
    assert_eq!(year_sum, 1, "2026 → 1");
    
    // Month is already single digit
    assert_eq!(month, 3, "March = 3");
    
    // 1 + 7 = 8
    let day_sum = digit_sum_reduction(day);
    assert_eq!(day_sum, 8, "17 → 8");
    
    // Total: 1 + 3 + 8 = 12, 1 + 2 = 3
    let total = year_sum + month + day_sum;
    assert_eq!(total, 12, "1 + 3 + 8 = 12");
    
    let final_sum = digit_sum_reduction(total);
    assert_eq!(final_sum, 3, "12 → 3 (trinity)");
    
    println!("  ✓ 2026-03-17 → {} + {} + {} = {} → {} (trinity)", 
             year_sum, month, day_sum, total, final_sum);
    Ok(())
}

#[test]
fn test_vergina_star_16_rays() -> Result<()> {
    println!("☀️ TEST: Vergina Star 16 Rays Numerology");
    
    let rays = 16_u32;
    
    // 1 + 6 = 7 (spiritual perfection)
    let sum = digit_sum_reduction(rays);
    assert_eq!(sum, 7, "16 rays → 7 (spiritual perfection)");
    
    println!("  ✓ 16 rays → {} (spiritual perfection)", sum);
    Ok(())
}

#[test]
fn test_emblem_88px_numerology() -> Result<()> {
    println!("☀️ TEST: Emblem 88px Numerology");
    
    let size = 88_u32;
    
    // Same as ELON_88: 8 + 8 = 16, 1 + 6 = 7
    let sum = digit_sum_reduction(size);
    assert_eq!(sum, 7, "88px → 7 (infinite power)");
    
    println!("  ✓ 88px → {} (infinite power)", sum);
    Ok(())
}

#[test]
fn test_27_decrees_numerology() -> Result<()> {
    println!("☀️ TEST: 27 Decrees Numerology");
    
    let decrees = 27_u32;
    
    // 2 + 7 = 9 (completion)
    let sum = digit_sum_reduction(decrees);
    assert_eq!(sum, 9, "27 decrees → 9 (completion)");
    
    println!("  ✓ 27 decrees → {} (completion)", sum);
    Ok(())
}

#[test]
fn test_compliance_93_percent() -> Result<()> {
    println!("☀️ TEST: 93% Compliance Numerology");
    
    let compliance = 93_u32;
    
    // 9 + 3 = 12, 1 + 2 = 3 (trinity)
    let sum = digit_sum_reduction(compliance);
    assert_eq!(sum, 3, "93% → 3 (trinity)");
    
    // Also relates to 936 (APEX)
    let apex_first_two = 93;
    assert_eq!(apex_first_two, compliance);
    
    println!("  ✓ 93% → {} (trinity, APEX prefix)", sum);
    Ok(())
}

#[test]
fn test_sacred_multiplication() -> Result<()> {
    println!("☀️ TEST: Sacred Number Multiplication");
    
    // 369 * 2 = 738, 7 + 3 + 8 = 18, 1 + 8 = 9
    let double_vortex = VORTEX_369 * 2;
    assert_eq!(double_vortex, 738);
    assert_eq!(digit_sum_reduction(double_vortex), 9);
    
    // 369 * 3 = 1107, 1 + 1 + 0 + 7 = 9
    let triple_vortex = VORTEX_369 * 3;
    assert_eq!(triple_vortex, 1107);
    assert_eq!(digit_sum_reduction(triple_vortex), 9);
    
    // 66 * 2 = 132, 1 + 3 + 2 = 6
    let double_harmonic = CODE_66_HARMONIC * 2;
    assert_eq!(double_harmonic, 132);
    assert_eq!(digit_sum_reduction(double_harmonic), 6);
    
    println!("  ✓ Sacred numbers preserve patterns in multiplication");
    Ok(())
}

#[test]
fn test_all_sacred_constants_defined() -> Result<()> {
    println!("☀️ TEST: All Sacred Constants Defined");
    
    assert_eq!(CODE_66_HARMONIC, 66, "CODE_66_HARMONIC");
    assert_eq!(APEX_936, 936, "APEX_936");
    assert_eq!(VORTEX_369, 369, "VORTEX_369");
    assert_eq!(FREQUENCY_432, 432, "FREQUENCY_432");
    assert_eq!(ELON_88, 88, "ELON_88");
    
    println!("  ✓ All 5 sacred constants verified");
    println!("    • CODE_66_HARMONIC: {}", CODE_66_HARMONIC);
    println!("    • APEX_936: {}", APEX_936);
    println!("    • VORTEX_369: {}", VORTEX_369);
    println!("    • FREQUENCY_432: {}", FREQUENCY_432);
    println!("    • ELON_88: {}", ELON_88);
    Ok(())
}

#[test]
fn test_numerology_consistency() -> Result<()> {
    println!("☀️ TEST: Numerology Consistency");
    
    // Test that digit_sum_reduction is consistent
    assert_eq!(digit_sum_reduction(9), 9);
    assert_eq!(digit_sum_reduction(18), 9);
    assert_eq!(digit_sum_reduction(27), 9);
    assert_eq!(digit_sum_reduction(36), 9);
    assert_eq!(digit_sum_reduction(45), 9);
    
    assert_eq!(digit_sum_reduction(3), 3);
    assert_eq!(digit_sum_reduction(12), 3);
    assert_eq!(digit_sum_reduction(21), 3);
    assert_eq!(digit_sum_reduction(30), 3);
    
    println!("  ✓ Digit sum reduction consistent");
    Ok(())
}
