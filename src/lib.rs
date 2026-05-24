#![doc = include_str!("../README.md")]

use proc_macro::*;
use Spacing::*;
use tt_path::TtPath;

fn stream<I>(iter: I) -> TokenStream
where I: IntoIterator<Item = TokenTree>,
{
    TokenStream::from_iter(iter)
}

fn span_setter(span: Span) -> impl Fn(TokenTree) -> TokenTree {
    move |mut tt| {
        tt.set_span(span);
        tt
    }
}

fn err(msg: &str, span: Span) -> TokenStream {
    let s = span_setter(span);
    stream([
        s(Punct::new(':', Joint).into()),
        s(Punct::new(':', Joint).into()),
        s(Ident::new("core", span).into()),
        s(Punct::new(':', Joint).into()),
        s(Punct::new(':', Joint).into()),
        s(Ident::new("compile_error", span).into()),
        s(Punct::new('!', Joint).into()),
        s(Group::new(Delimiter::Brace, stream([
            s(Literal::string(msg).into()),
        ])).into()),
    ])
}

struct Sorter {
    key: TtPath,
    prefix: TokenStream,
    tokens: Vec<TokenTree>,
    args: Group,
}
impl Sorter {
    fn finish(self) -> TokenStream {
        let mut args = self.args.stream().into_iter().collect::<Vec<_>>();
        args.sort_by_key(|tt| {
            self.key.apply(tt.clone())
                .into_iter()
                .filter_map(Result::ok)
                .map(|it| it.to_string())
                .collect::<Vec<_>>()
        });
        args.splice(..0, self.prefix);

        let mut sorted_args = Group::new(self.args.delimiter(), TokenStream::from_iter(args));
        sorted_args.set_span(self.args.span());

        self.tokens.into_iter()
            .chain([sorted_args.into()])
            .collect()
    }
}

#[doc = include_str!("../README.md")]
#[proc_macro]
pub fn sort_in(tokens: TokenStream) -> TokenStream {
    let mut tokens = tokens.into_iter().collect::<Vec<_>>();
    let Some(args) = tokens.pop() else {
        return err("must input a `()` `{}` `[]`", Span::call_site());
    };
    let TokenTree::Group(args) = args else {
        return err("last token must is group", args.span());
    };

    let mut prefix = TokenStream::new();
    let mut key = TtPath::from_tokens(&mut None.into_iter().peekable()).unwrap();

    if let Some(TokenTree::Group(first)) = tokens.first()
        && first.delimiter() == Delimiter::Bracket
    {
        let iter = &mut first.stream().into_iter().peekable();
        match TtPath::from_tokens(iter) {
            Ok(tt_path) => key = tt_path,
            Err(e) => return err(e.msg(), e.span()),
        }
        if let Some(peek) = iter.peek() {
            return err("unexpected key token", peek.span());
        }
        tokens.remove(0);
    }

    if let Some(TokenTree::Group(first)) = tokens.first()
        && first.delimiter() == Delimiter::Brace
    {
        prefix = first.stream();
        tokens.remove(0);
    }

    Sorter { key, prefix, tokens, args }.finish()
}
