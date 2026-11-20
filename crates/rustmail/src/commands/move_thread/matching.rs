use serenity::all::ChannelId;

pub fn find_best_match_category(
    target_name: &str,
    categories: &[(ChannelId, String)],
) -> Option<(ChannelId, String)> {
    let target_lower = target_name.to_lowercase();
    let mut best_match = None;
    let mut best_distance = usize::MAX;

    for (id, name) in categories {
        let name_lower = name.to_lowercase();

        if name_lower == target_lower {
            return Some((*id, name.clone()));
        }

        if name_lower.contains(&target_lower) || target_lower.contains(&name_lower) {
            let distance = levenshtein_distance(&target_lower, &name_lower);
            if distance < best_distance {
                best_distance = distance;
                best_match = Some((*id, name.clone()));
            }
        }
    }

    if let Some((id, name)) = best_match {
        let max_distance = (target_name.len().max(name.len()) as f64 * 0.7) as usize;
        if best_distance <= max_distance {
            return Some((id, name));
        }
    }

    None
}

pub fn levenshtein_distance(s1: &str, s2: &str) -> usize {
    let len1 = s1.chars().count();
    let len2 = s2.chars().count();

    if len1 == 0 {
        return len2;
    }
    if len2 == 0 {
        return len1;
    }

    let mut matrix = vec![vec![0; len2 + 1]; len1 + 1];

    for (i, row) in matrix.iter_mut().enumerate() {
        row[0] = i;
    }
    for (j, cell) in matrix[0].iter_mut().enumerate() {
        *cell = j;
    }

    let chars1: Vec<char> = s1.chars().collect();
    let chars2: Vec<char> = s2.chars().collect();

    for i in 1..=len1 {
        for j in 1..=len2 {
            let cost = if chars1[i - 1] == chars2[j - 1] { 0 } else { 1 };
            matrix[i][j] = (matrix[i - 1][j] + 1)
                .min(matrix[i][j - 1] + 1)
                .min(matrix[i - 1][j - 1] + cost);
        }
    }

    matrix[len1][len2]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn categories() -> Vec<(ChannelId, String)> {
        vec![
            (ChannelId::new(1), "Support".to_string()),
            (ChannelId::new(2), "Billing support".to_string()),
            (ChannelId::new(3), "Archives".to_string()),
        ]
    }

    #[test]
    fn exact_match_ignores_case() {
        assert_eq!(
            find_best_match_category("archives", &categories()).map(|(id, _)| id),
            Some(ChannelId::new(3))
        );
    }

    #[test]
    fn partial_match_picks_the_closest_name() {
        assert_eq!(
            find_best_match_category("billing", &categories()).map(|(id, _)| id),
            Some(ChannelId::new(2))
        );
    }

    #[test]
    fn unrelated_name_matches_nothing() {
        assert_eq!(find_best_match_category("zzz", &categories()), None);
    }

    #[test]
    fn distances() {
        assert_eq!(levenshtein_distance("kitten", "sitting"), 3);
        assert_eq!(levenshtein_distance("", "abc"), 3);
        assert_eq!(levenshtein_distance("same", "same"), 0);
    }
}
