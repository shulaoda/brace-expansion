#[derive(Debug)]
pub enum GlobNode {
  None,
  Range(usize, usize),
  Brace(usize, Vec<GlobNode>),
  Pattern(Vec<GlobNode>),
}

fn scan_capacity(glob: &[u8], mut start: usize, end: usize, stop_with_brace_end: bool) -> usize {
  let mut count = 0;

  let mut braces = 0;
  let mut in_brackets = false;
  let mut pattern_extend = false;

  while start < end {
    match glob[start] {
      b'\\' => start += 1,
      b'[' => in_brackets = true,
      b']' => in_brackets = false,
      b',' if !in_brackets && braces == 1 && stop_with_brace_end => count += 1,
      b'{' if !in_brackets => braces += 1,
      b'}' if !in_brackets && braces > 0 => {
        braces -= 1;

        if braces == 0 {
          count += 1;

          if stop_with_brace_end {
            return count;
          }

          pattern_extend = false;
        }
      }
      _ => {
        if braces == 0 && !pattern_extend {
          count += 1;
          pattern_extend = true;
        }
      }
    }
    start += 1;
  }

  count
}

fn parse_pattern(glob: &[u8], mut start: usize, end: usize) -> GlobNode {
  let capacity = scan_capacity(glob, start, end, false);

  let mut pattern: Vec<GlobNode> = Vec::with_capacity(capacity);
  let mut pattern_extend = false;

  let mut braces = 0;
  let mut in_brackets = false;
  let mut brace_pattern_count = 1;
  let mut brace_pattern_start = start;

  while start < end {
    match glob[start] {
      b'\\' => start += 1,
      b'[' => in_brackets = true,
      b']' => in_brackets = false,
      b'{' if !in_brackets => {
        braces += 1;

        if braces == 1 {
          pattern_extend = false;
          brace_pattern_count = 1;
          brace_pattern_start = start + 1;
        }
      }
      b'}' if !in_brackets && braces > 0 => {
        braces -= 1;

        if braces == 0 {
          let node = if start > brace_pattern_start {
            parse_pattern(glob, brace_pattern_start, start)
          } else {
            GlobNode::None
          };

          if brace_pattern_count == 1 {
            pattern.push(node);
          } else {
            if let Some(GlobNode::Brace(_, ref mut brace)) = pattern.last_mut() {
              brace.push(node);
            }
          }

          pattern_extend = false;
        }
      }
      b',' if !in_brackets && braces == 1 => {
        let node = if start > brace_pattern_start {
          parse_pattern(glob, brace_pattern_start, start)
        } else {
          GlobNode::None
        };

        if pattern_extend {
          if let Some(GlobNode::Brace(_, ref mut brace)) = pattern.last_mut() {
            brace.push(node);
          }
        } else {
          brace_pattern_count = scan_capacity(glob, brace_pattern_start - 1, end, true);
          let mut brace: Vec<GlobNode> = Vec::with_capacity(brace_pattern_count);

          brace.push(node);
          pattern_extend = true;
          pattern.push(GlobNode::Brace(0, brace));
        }

        brace_pattern_start = start + 1;
      }
      _ => {
        if braces == 0 {
          if pattern_extend {
            if let Some(GlobNode::Range(_, ref mut end)) = pattern.last_mut() {
              *end += 1;
            }
          } else {
            pattern_extend = true;
            pattern.push(GlobNode::Range(start, start + 1));
          }
        }
      }
    }
    start += 1;
  }

  match pattern.len() {
    0 => GlobNode::None,
    1 => pattern.pop().unwrap(),
    _ => GlobNode::Pattern(pattern),
  }
}

pub fn parse_brace(glob: &[u8]) -> GlobNode {
  parse_pattern(glob, 0, glob.len())
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test() {
    println!(
      "{:?}",
      parse_brace("some/**/{a,b{c,de},{g,l}}.js".as_bytes())
    );
  }
}
