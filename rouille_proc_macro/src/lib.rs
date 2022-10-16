use proc_macro::{Group, Ident, TokenStream, TokenTree};

fn replace_ident(ident: Ident) -> Option<TokenTree> {
    let ident_str = ident.to_string();

    let new_str = match ident_str.as_str() {
        "Fuck" => "Err",
        "Good" => "Ok",
        "taxte" => "String",
        "dbshit" => "HashMap",
        "normal" => "Default",
        "CALISSE" => "Error",
        "JeSaisTu" => "Option",
        "kelke" => "Some",
        "FuckAll" => "None",
        "CqueCaDonne" => "Result",
        "Moi" => "Self",
        "printdon" => "println",
        "STOPCRISS" => "break",
        "sametime" => "async",
        "attendcriss" => "await",
        "refait" => "loop",
        "decriss" => "move",
        "boite" => "crate",
        "jvoispas" => "unreachable_code",
        "same" => "as",
        "bougepas" => "const",
        "convention" => "trait",
        "dangereux" => "unsafe",
        "de" => "in",
        "depuis" => "from",
        "dynamique" => "dyn",
        "déballer" => "unwrap",
        "défaut" => "default",
        "en_réf" => "as_ref",
        "es" => "io",
        "externe" => "extern",
        "faux" => "false",
        "fonction" => "fn",
        "génial" => "super",
        "insérer" => "insert",
        "lire" => "get",
        "légal" => "allow",
        "merde" | "calisse" | "oups" => "panic",
        "module" => "mod",
        "mutable" => "mut",
        "nouveau" => "new",
        "où" => "where",
        "pour" => "for",
        "prendre_ou_insérer_avec" => "get_or_insert_with",
        "principale" => "main",
        "public" => "pub",
        "que" => None?,
        "renvoie" => "return",
        "réalisation" => "impl",
        "réf" => "ref",
        "selon" => "match",
        "si" => "if",
        "sinon" => "else",
        "soi" => "self",
        "soit" => "let",
        "statique" => "static",
        "structure" => "struct",
        "suppose" => "expect",
        "tant" => "while",
        "utilisons" => "use",
        "vers" => "into",
        "vrai" => "true",
        "énumération" => "enum",

        _ => &ident_str,
    };

    let new_ident = Ident::new(new_str, ident.span());
    Some(TokenTree::Ident(new_ident))
}

fn replace_tree(tok: TokenTree, out: &mut Vec<TokenTree>) {
    match tok {
        TokenTree::Group(group) => {
            let mut group_elem = Vec::new();
            replace_stream(group.stream(), &mut group_elem);
            let mut new_stream = TokenStream::new();
            new_stream.extend(group_elem);
            out.push(TokenTree::Group(Group::new(group.delimiter(), new_stream)));
        }
        TokenTree::Ident(ident) => {
            if let Some(ident) = replace_ident(ident) {
                out.push(ident);
            }
        }
        TokenTree::Punct(..) | TokenTree::Literal(..) => {
            out.push(tok);
        }
    }
}

fn replace_stream(ts: TokenStream, out: &mut Vec<TokenTree>) {
    for tok in ts {
        replace_tree(tok, out)
    }
}

#[proc_macro]
pub fn rouille(item: TokenStream) -> TokenStream {
    let mut returned = Vec::new();
    replace_stream(item, &mut returned);
    let mut out = TokenStream::new();
    out.extend(returned);
    out
}
