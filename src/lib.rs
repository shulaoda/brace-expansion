pub struct Pattern {
  pub value: Vec<u8>,
  branch: Vec<(u8, u8)>,
  shadow: Vec<(usize, usize)>,
}

impl Pattern {
  pub fn with(glob: &[u8]) -> Option<Self> {
    Self::brace(glob).map(|branch| {
      let value = Vec::with_capacity(glob.len());
      let shadow = Vec::new();
      let mut node = Pattern {
        value,
        branch,
        shadow,
      };
      node.track(glob);
      node
    })
  }

  pub fn brace(glob: &[u8]) -> Option<Vec<(u8, u8)>> {
    let mut braces = 0;
    let mut current = 0;
    let mut in_brackets = false;

    let mut stack = [0; 10];
    let mut branch = Vec::<(u8, u8)>::new();

    while current < glob.len() {
      match glob[current] {
        b'\\' => current += 1,
        b']' if in_brackets => in_brackets = false,
        b'[' if !in_brackets => in_brackets = true,
        b',' if !in_brackets && braces > 0 => {
          branch[stack[braces - 1]].1 += 1;
        }
        b'}' if !in_brackets && braces > 0 => {
          braces -= 1;
        }
        b'{' if !in_brackets => {
          branch.push((0, 1));

          stack[braces] = branch.len() - 1;
          braces += 1;
        }
        _ => {}
      }
      current += 1;
    }

    if braces == 0 && !in_brackets {
      Some(branch)
    } else {
      None
    }
  }

  pub fn track(&mut self, glob: &[u8]) {
    let mut index = 0;

    let mut braces = 0;
    let mut current = 0;
    let mut is_valid = true;
    let mut in_brackets = false;

    let mut len = 0;
    let mut stack: [(u8, usize); 10] = [(0, 0); 10];

    self.value.clear();
    while current < glob.len() {
      match glob[current] {
        b',' if !in_brackets && braces > 0 => {
          if len == braces {
            let (i, idx) = &mut stack[len - 1];

            *i += 1;
            is_valid = self.branch[*idx].0 == *i;
          }
        }
        b'}' if !in_brackets && braces > 0 => {
          if len == braces {
            len -= 1;
            is_valid = true;
          }
          braces -= 1;
        }
        b'{' if !in_brackets => {
          if is_valid {
            stack[len] = (0, index);

            len += 1;
            is_valid = self.branch[index].0 == 0;

            self.shadow.push((index, self.value.len()));
          }

          braces += 1;
          index += 1;
        }
        c => {
          if is_valid {
            self.value.push(c);
          }

          if c == b'\\' {
            current += 1;
            if is_valid && current < glob.len() {
              self.value.push(glob[current]);
            }
          } else if c == b']' && in_brackets {
            in_brackets = false;
          } else if c == b'[' && !in_brackets {
            in_brackets = true;
          }
        }
      }

      current += 1;
    }
  }

  pub fn trigger(&mut self, glob: &[u8], target: usize) -> bool {
    while let Some((idx, position)) = self.shadow.pop() {
      if target >= position {
        self.branch[idx].0 += 1;
        if self.branch[idx].1 != self.branch[idx].0 {
          self.shadow.clear();
          self.track(glob);
          return true;
        }
        self.branch[idx].0 = 0;
      }
    }
    false
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test() {
    let glob = b"some/{,b{c,d}f,e}/ccc.{png,jpg}";
    let mut node = Pattern::with(glob).unwrap();

    println!("{}", node.trigger(glob, node.value.len()));
    println!("{:?}", String::from_utf8(node.value));
  }

  #[test]
  fn test2() {
    let glob = b"some/{a,b{c,d}f,e}/ccc.{png,jpg}";
    let mut node = Pattern::with(glob).unwrap();

    while node.trigger(glob, node.value.len()) {
      println!("{:?}", String::from_utf8(node.value.clone()));
    }
  }

  #[test]
  fn create_glob_test() {
    let glob = b"some/{a,b{c,d}f,e}/ccc.{png,jpg}";
    if let Some(node) = Pattern::with(glob) {
      println!("{:?}", String::from_utf8(node.value));
    }
  }
}
