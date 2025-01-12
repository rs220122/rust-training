// トライ木の実装

const ALPHABET_SIZE: usize = 26;

#[derive(Debug, Default)]
struct TrieNode {
    // 子ノードを配列で管理
    children: [Option<Box<TrieNode>>; ALPHABET_SIZE],
    // 単語の終端かどうか
    is_end_of_word: bool,
}

impl TrieNode {
    fn new() -> Self {
        Self {
            children: Default::default(),
            is_end_of_word: false,
        }
    }
}

#[derive(Debug)]
struct Trie {
    root: TrieNode,
}

impl Trie {
    fn new() -> Self {
        Self {
            root: TrieNode::new(),
        }
    }

    // 文字をインデックスに変換
    fn char_to_index(ch: char) -> usize {
        // 小文字に変換して、アルファベットのインデックスを取得
        (ch.to_ascii_lowercase() as usize) - ('a' as usize)
    }

    // 単語を挿入
    fn insert(&mut self, word: &str) {
        let mut node = &mut self.root;
        for ch in word.chars() {
            let index = Trie::char_to_index(ch);
            if node.children[index].is_none() {
                // 子ノードが存在しない場合は新規作成
                node.children[index] = Some(Box::new(TrieNode::new()));
            }
            node = node.children[index].as_mut().unwrap();
        }
        node.is_end_of_word = true;
    }

    // 単語が存在するかどうか
    fn search(&self, word: &str) -> bool {
        let mut node = &self.root;
        for ch in word.chars() {
            let index = Trie::char_to_index(ch);

            if let Some(ref child) = node.children[index] {
                node = child.as_ref();
            } else {
                return false;
            }
        }
        node.is_end_of_word
    }

    // 接頭辞が存在するかどうか
    fn starts_with(&self, prefix: &str) -> bool {
        let mut node = &self.root;
        for ch in prefix.chars() {
            let index = Trie::char_to_index(ch);
            if let Some(ref child) = node.children[index] {
                node = child.as_ref();
            } else {
                return false;
            }
        }
        true
    }
}

fn main() {
    let mut trie = Trie::new();

    trie.insert("Programming");
    trie.insert("Language");
    trie.insert("Program");
    trie.insert("Programmer");

    println!("{}", trie.search("Program")); // true
    println!("{}", trie.search("Programmer")); // true
    println!("{}", trie.search("Programm")); // false
    println!("{}", trie.search("Language")); // true
    println!("{}", trie.search("Languages")); // false
}
