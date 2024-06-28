#[derive(Debug)]
enum GlobNode {
  None,
  Range(usize, usize),
  Brace(usize, Vec<GlobNode>),
  Pattern(Vec<GlobNode>),
}

fn parse_pattern(glob: &[u8], start: usize, end: usize) -> GlobNode {
  let mut pattern: Vec<GlobNode> = vec![];
  let mut pattern_extend = false;

  let mut current = start;

  let mut braces = 0;
  let mut in_brackets = false;
  let mut brace_pattern_index = 0;
  let mut brace_pattern_start = current;

  while current < end {
    match glob[current] {
      b'[' => in_brackets = true,
      b']' => in_brackets = false,
      b'\\' => current += 1,
      b'{' if !in_brackets => {
        braces += 1;

        if braces == 1 {
          pattern_extend = false;
          brace_pattern_start = current + 1;
        }
      }
      b'}' if !in_brackets && braces > 0 => {
        braces -= 1;

        if braces == 0 {
          let node = if current > brace_pattern_start {
            parse_pattern(glob, brace_pattern_start, current)
          } else {
            GlobNode::None
          };

          if brace_pattern_index == 0 {
            pattern.push(node);
          } else {
            if let Some(GlobNode::Brace(_, brace)) = pattern.last_mut() {
              brace.push(node);
            }
          }

          pattern_extend = false;
        }
      }
      b',' if !in_brackets && braces == 1 => {
        let node = if current > brace_pattern_start {
          parse_pattern(glob, brace_pattern_start, current)
        } else {
          GlobNode::None
        };

        if pattern_extend {
          if let Some(GlobNode::Brace(_, brace)) = pattern.last_mut() {
            brace.push(node);
          }
        } else {
          pattern.push(GlobNode::Brace(0, vec![node]));
          pattern_extend = true;
        }

        brace_pattern_index += 1;
        brace_pattern_start = current + 1;
      }
      _ if braces == 0 => {
        if pattern_extend {
          if let Some(GlobNode::Range(_, end)) = pattern.last_mut() {
            *end += 1;
          }
        } else {
          pattern_extend = true;
          pattern.push(GlobNode::Range(current, current + 1));
        }
      }
      _ => {}
    }
    current += 1;
  }

  match pattern.len() {
    0 => GlobNode::None,
    1 => pattern.pop().unwrap(),
    _ => GlobNode::Pattern(pattern),
  }
}

fn main() {
  let glob = "abc{a,b,c{e,f},{d,d}}d".as_bytes();
  println!("{:?}", parse_pattern(glob, 0, glob.len()));
}
