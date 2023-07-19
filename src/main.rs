#[cfg(test)]
mod tests;

//
// TTTTTTTTTTTTTTTTTTTTTTTHHHHHHHHH     HHHHHHHHHEEEEEEEEEEEEEEEEEEEEEE
// T:::::::::::::::::::::TH:::::::H     H:::::::HE::::::::::::::::::::E
// T:::::::::::::::::::::TH:::::::H     H:::::::HE::::::::::::::::::::E
// T:::::TT:::::::TT:::::THH::::::H     H::::::HHEE::::::EEEEEEEEE::::E
// TTTTTT  T:::::T  TTTTTT  H:::::H     H:::::H    E:::::E       EEEEEE
//         T:::::T          H:::::H     H:::::H    E:::::E
//         T:::::T          H::::::HHHHH::::::H    E::::::EEEEEEEEEE
//         T:::::T          H:::::::::::::::::H    E:::::::::::::::E
//         T:::::T          H:::::::::::::::::H    E:::::::::::::::E
//         T:::::T          H::::::HHHHH::::::H    E::::::EEEEEEEEEE
//         T:::::T          H:::::H     H:::::H    E:::::E
//         T:::::T          H:::::H     H:::::H    E:::::E       EEEEEE
//       TT:::::::TT      HH::::::H     H::::::HHEE::::::EEEEEEEE:::::E
//       T:::::::::T      H:::::::H     H:::::::HE::::::::::::::::::::E
//       T:::::::::T      H:::::::H     H:::::::HE::::::::::::::::::::E
//       TTTTTTTTTTT      HHHHHHHHH     HHHHHHHHHEEEEEEEEEEEEEEEEEEEEEE
//
//    SSSSSSSSSSSSSSS UUUUUUUU     UUUUUUUUPPPPPPPPPPPPPPPPP   EEEEEEEEEEEEEEEEEEEEEERRRRRRRRRRRRRRRRR
//  SS:::::::::::::::SU::::::U     U::::::UP::::::::::::::::P  E::::::::::::::::::::ER::::::::::::::::R
// S:::::SSSSSS::::::SU::::::U     U::::::UP::::::PPPPPP:::::P E::::::::::::::::::::ER::::::RRRRRR:::::R
// S:::::S     SSSSSSSUU:::::U     U:::::UUPP:::::P     P:::::PEE::::::EEEEEEEEE::::ERR:::::R     R:::::R
// S:::::S             U:::::U     U:::::U   P::::P     P:::::P  E:::::E       EEEEEE  R::::R     R:::::R
// S:::::S             U:::::U     U:::::U   P::::P     P:::::P  E:::::E               R::::R     R:::::R
//  S::::SSSS          U:::::U     U:::::U   P::::PPPPPP:::::P   E::::::EEEEEEEEEE     R::::RRRRRR:::::R
//   SS::::::SSSSS     U:::::U     U:::::U   P:::::::::::::PP    E:::::::::::::::E     R:::::::::::::RR
//     SSS::::::::SS   U:::::U     U:::::U   P::::PPPPPPPPP      E:::::::::::::::E     R::::RRRRRR:::::R
//        SSSSSS::::S  U:::::U     U:::::U   P::::P              E::::::EEEEEEEEEE     R::::R     R:::::R
//             S:::::S U:::::U     U:::::U   P::::P              E:::::E               R::::R     R:::::R
//             S:::::S U::::::U   U::::::U   P::::P              E:::::E       EEEEEE  R::::R     R:::::R
// SSSSSSS     S:::::S U:::::::UUU:::::::U PP::::::PP          EE::::::EEEEEEEE:::::ERR:::::R     R:::::R
// S::::::SSSSSS:::::S  UU:::::::::::::UU  P::::::::P          E::::::::::::::::::::ER::::::R     R:::::R
// S:::::::::::::::SS     UU:::::::::UU    P::::::::P          E::::::::::::::::::::ER::::::R     R:::::R
//  SSSSSSSSSSSSSSS         UUUUUUUUU      PPPPPPPPPP          EEEEEEEEEEEEEEEEEEEEEERRRRRRRR     RRRRRRR
//
// TTTTTTTTTTTTTTTTTTTTTTTIIIIIIIIIINNNNNNNN        NNNNNNNNYYYYYYY       YYYYYYY
// T:::::::::::::::::::::TI::::::::IN:::::::N       N::::::NY:::::Y       Y:::::Y
// T:::::::::::::::::::::TI::::::::IN::::::::N      N::::::NY:::::Y       Y:::::Y
// T:::::TT:::::::TT:::::TII::::::IIN:::::::::N     N::::::NY::::::Y     Y::::::Y
// TTTTTT  T:::::T  TTTTTT  I::::I  N::::::::::N    N::::::NYYY:::::Y   Y:::::YYY
//         T:::::T          I::::I  N:::::::::::N   N::::::N   Y:::::Y Y:::::Y
//         T:::::T          I::::I  N:::::::N::::N  N::::::N    Y:::::Y:::::Y
//         T:::::T          I::::I  N::::::N N::::N N::::::N     Y:::::::::Y
//         T:::::T          I::::I  N::::::N  N::::N:::::::N      Y:::::::Y
//         T:::::T          I::::I  N::::::N   N:::::::::::N       Y:::::Y
//         T:::::T          I::::I  N::::::N    N::::::::::N       Y:::::Y
//         T:::::T          I::::I  N::::::N     N:::::::::N       Y:::::Y
//       TT:::::::TT      II::::::IIN::::::N      N::::::::N       Y:::::Y
//       T:::::::::T      I::::::::IN::::::N       N:::::::N    YYYY:::::YYYY
//       T:::::::::T      I::::::::IN::::::N        N::::::N    Y:::::::::::Y
//       TTTTTTTTTTT      IIIIIIIIIINNNNNNNN         NNNNNNN    YYYYYYYYYYYYY
//
//         CCCCCCCCCCCCC     OOOOOOOOO     MMMMMMMM               MMMMMMMMPPPPPPPPPPPPPPPPP   IIIIIIIIIILLLLLLLLLLL             EEEEEEEEEEEEEEEEEEEEEERRRRRRRRRRRRRRRRR
//      CCC::::::::::::C   OO:::::::::OO   M:::::::M             M:::::::MP::::::::::::::::P  I::::::::IL:::::::::L             E::::::::::::::::::::ER::::::::::::::::R
//    CC:::::::::::::::C OO:::::::::::::OO M::::::::M           M::::::::MP::::::PPPPPP:::::P I::::::::IL:::::::::L             E::::::::::::::::::::ER::::::RRRRRR:::::R
//   C:::::CCCCCCCC::::CO:::::::OOO:::::::OM:::::::::M         M:::::::::MPP:::::P     P:::::PII::::::IILL:::::::LL             EE::::::EEEEEEEEE::::ERR:::::R     R:::::R
//  C:::::C       CCCCCCO::::::O   O::::::OM::::::::::M       M::::::::::M  P::::P     P:::::P  I::::I    L:::::L                 E:::::E       EEEEEE  R::::R     R:::::R
// C:::::C              O:::::O     O:::::OM:::::::::::M     M:::::::::::M  P::::P     P:::::P  I::::I    L:::::L                 E:::::E               R::::R     R:::::R
// C:::::C              O:::::O     O:::::OM:::::::M::::M   M::::M:::::::M  P::::PPPPPP:::::P   I::::I    L:::::L                 E::::::EEEEEEEEEE     R::::RRRRRR:::::R
// C:::::C              O:::::O     O:::::OM::::::M M::::M M::::M M::::::M  P:::::::::::::PP    I::::I    L:::::L                 E:::::::::::::::E     R:::::::::::::RR
// C:::::C              O:::::O     O:::::OM::::::M  M::::M::::M  M::::::M  P::::PPPPPPPPP      I::::I    L:::::L                 E:::::::::::::::E     R::::RRRRRR:::::R
// C:::::C              O:::::O     O:::::OM::::::M   M:::::::M   M::::::M  P::::P              I::::I    L:::::L                 E::::::EEEEEEEEEE     R::::R     R:::::R
// C:::::C              O:::::O     O:::::OM::::::M    M:::::M    M::::::M  P::::P              I::::I    L:::::L                 E:::::E               R::::R     R:::::R
//  C:::::C       CCCCCCO::::::O   O::::::OM::::::M     MMMMM     M::::::M  P::::P              I::::I    L:::::L         LLLLLL  E:::::E       EEEEEE  R::::R     R:::::R
//   C:::::CCCCCCCC::::CO:::::::OOO:::::::OM::::::M               M::::::MPP::::::PP          II::::::IILL:::::::LLLLLLLLL:::::LEE::::::EEEEEEEE:::::ERR:::::R     R:::::R
//    CC:::::::::::::::C OO:::::::::::::OO M::::::M               M::::::MP::::::::P          I::::::::IL::::::::::::::::::::::LE::::::::::::::::::::ER::::::R     R:::::R
//      CCC::::::::::::C   OO:::::::::OO   M::::::M               M::::::MP::::::::P          I::::::::IL::::::::::::::::::::::LE::::::::::::::::::::ER::::::R     R:::::R
//         CCCCCCCCCCCCC     OOOOOOOOO     MMMMMMMM               MMMMMMMMPPPPPPPPPP          IIIIIIIIIILLLLLLLLLLLLLLLLLLLLLLLLEEEEEEEEEEEEEEEEEEEEEERRRRRRRR     RRRRRRR
//
// =======================================================================================================================================================================
// =======================================================================================================================================================================
// =======================================================================================================================================================================
// =======================================================================================================================================================================

//
// Today we're going to write a compiler together. But not just any compiler... A
// super duper teeny tiny compiler! A compiler that is so small that if you
// remove all the comments this file would only be ~200 lines of actual code.
//
// We're going to compile some lisp-like function calls into some C-like
// function calls.
//
// If you are not familiar with one or the other. I'll just give you a quick intro.
//
// If we had two functions `add` and `subtract` they would be written like this:
//
//                  LISP                      C
//
//   2 + 2          (add 2 2)                 add(2, 2)
//   4 - 2          (subtract 4 2)            subtract(4, 2)
//   2 + (4 - 2)    (add 2 (subtract 4 2))    add(2, subtract(4, 2))
//
// Easy peezy right?
//
// Well good, because this is exactly what we are going to compile. While this
// is neither a complete LISP or C syntax, it will be enough of the syntax to
// demonstrate many of the major pieces of a modern compiler.
//
//*
// Most compilers break down into three primary stages: Parsing, Transformation,
// and Code Generation
//
// 1. *Parsing* is taking raw code and turning it into a more abstract
//    representation of the code.
//
// 2. *Transformation* takes this abstract representation and manipulates to do
//    whatever the compiler wants it to.
//
// 3. *Code Generation* takes the transformed representation of the code and
//    turns it into new code.

// Parsing
// -------
//
// Parsing typically gets broken down into two phases: Lexical Analysis and
// Syntactic Analysis.
//
// 1. *Lexical Analysis* takes the raw code and splits it apart into these things
//    called tokens by a thing called a tokenizer (or lexer).
//
//    Tokens are an array of tiny little objects that describe an isolated piece
//    of the syntax. They could be numbers, labels, punctuation, operators,
//    whatever.
//
// 2. *Syntactic Analysis* takes the tokens and reformats them into a
//    representation that describes each part of the syntax and their relation
//    to one another. This is known as an intermediate representation or
//    Abstract Syntax Tree.
//
//    An Abstract Syntax Tree, or AST for short, is a deeply nested object that
//    represents code in a way that is both easy to work with and tells us a lot
//    of information.
//
// For the following syntax:
//
//   (add 2 (subtract 4 2))
//
// Tokens might look something like this:
//
//   [
//     { type: 'paren',  value: '('        },
//     { type: 'name',   value: 'add'      },
//     { type: 'number', value: '2'        },
//     { type: 'paren',  value: '('        },
//     { type: 'name',   value: 'subtract' },
//     { type: 'number', value: '4'        },
//     { type: 'number', value: '2'        },
//     { type: 'paren',  value: ')'        },
//     { type: 'paren',  value: ')'        }
//   ]
//
// And an Abstract Syntax Tree (AST) might look like this:
//
//   {
//     type: 'Program',
//     body: [{
//       type: 'CallExpression',
//       name: 'add',
//       params: [{
//         type: 'NumberLiteral',
//         value: '2'
//       }, {
//         type: 'CallExpression',
//         name: 'subtract',
//         params: [{
//           type: 'NumberLiteral',
//           value: '4'
//         }, {
//           type: 'NumberLiteral',
//           value: '2'
//         }]
//       }]
//     }]
//   }
//

// Transformation
// --------------
//
// The next type of stage for a compiler is transformation. Again, this just
// takes the AST from the last step and makes changes to it. It can manipulate
// the AST in the same language or it can translate it into an entirely new
// language.
//
// Let’s look at how we would transform an AST.
//
// You might notice that our AST has elements within it that look very similar.
// There are these objects with a type property. Each of these are known as an
// AST Node. These nodes have defined properties on them that describe one
// isolated part of the tree.
//
// We can have a node for a "NumberLiteral":
//
//   {
//     type: 'NumberLiteral',
//     value: '2'
//   }
//
// Or maybe a node for a "CallExpression":
//
//   {
//     type: 'CallExpression',
//     name: 'subtract',
//     params: [...nested nodes go here...]
//   }
//
// When transforming the AST we can manipulate nodes by
// adding/removing/replacing properties, we can add new nodes, remove nodes, or
// we could leave the existing AST alone and create an entirely new one based
// on it.
//
// Since we’re targeting a new language, we’re going to focus on creating an
// entirely new AST that is specific to the target language.

// Traversal
// ---------
//
// In order to navigate through all of these nodes, we need to be able to
// traverse through them. This traversal process goes to each node in the AST
// depth-first.
//
//   {
//     type: 'Program',
//     body: [{
//       type: 'CallExpression',
//       name: 'add',
//       params: [{
//         type: 'NumberLiteral',
//         value: '2'
//       }, {
//         type: 'CallExpression',
//         name: 'subtract',
//         params: [{
//           type: 'NumberLiteral',
//           value: '4'
//         }, {
//           type: 'NumberLiteral',
//           value: '2'
//         }]
//       }]
//     }]
//   }
//
// So for the above AST we would go:
//
//   1. Program - Starting at the top level of the AST
//   2. CallExpression (add) - Moving to the first element of the Program's body
//   3. NumberLiteral (2) - Moving to the first element of CallExpression's params
//   4. CallExpression (subtract) - Moving to the second element of CallExpression's params
//   5. NumberLiteral (4) - Moving to the first element of CallExpression's params
//   6. NumberLiteral (2) - Moving to the second element of CallExpression's params
//
// If we were manipulating this AST directly, instead of creating a separate AST,
// we would likely introduce all sorts of abstractions here. But just visiting
// each node in the tree is enough.
//
// The reason I use the word “visiting” is because there is this pattern of how
// to represent operations on elements of an object structure.
//
// Visitors
// --------
//
// The basic idea here is that we are going to create a “visitor” object that
// has methods that will accept different node types.
//
//   var visitor = {
//     NumberLiteral() {},
//     CallExpression() {}
//   };
//
// When we traverse our AST we will call the methods on this visitor whenever we
// encounter a node of a matching type.
//
// In order to make this useful we will also pass the node and a reference to
// the parent node.
//
//   var visitor = {
//     NumberLiteral(node, parent) {},
//     CallExpression(node, parent) {}
//   };

// Code Generation
// ---------------
//
// The final phase of a compiler is code generation. Sometimes compilers will do
// things that overlap with transformation, but for the most part code
// generation just means take our AST and string-ify code back out.
//
// Code generators work several different ways, some compilers will reuse the
// tokens from earlier, others will have created a separate representation of
// the code so that they can print node linearly, but from what I can tell most
// will use the same AST we just created, which is what we’re going to focus on.
//
// Effectively our code generator will know how to “print” all of the different
// node types of the AST, and it will recursively call itself to print nested
// nodes until everything is printed into one long string of code.

// And that's it! That's all the different pieces of a compiler.
//
// Now that isn’t to say every compiler looks exactly like I described here.
// Compilers serve many different purposes, and they might need more steps than
// I have detailed.
//
// But now you should have a general high-level idea of what most compilers look
// like.
//
// Now that I’ve explained all of this, you’re all good to go write your own
// compilers right?
//
// Just kidding, that's what I'm here to help with :P
//
// So let's begin...

// ============================================================================
//                                   (/^▽^)/
//                                THE TOKENIZER!
// ============================================================================
//

//
// We're gonna start off with our first phase of parsing, lexical analysis, with
// the tokenizer.
//
// We're just going to take our string of code and break it down into an array
// of tokens.
//
//   (add 2 (subtract 4 2))   =>   [{ type: 'paren', value: '(' }, ...]
//

/// An enum of possible token kinds
#[derive(Debug, PartialEq)]
enum TokenKind {
    Paren,
    Number,
    Name,
}

/// A struct containing information about a token
#[derive(Debug, PartialEq)]
struct Token {
    kind: TokenKind,
    value: String,
}

// We start by accepting an input string of code, and we're gonna set up two
// things...
fn tokenizer(input: String) -> Vec<Token> {
    // Append a new line to the input
    let mut input = input;
    input.push('\n');

    // A vec of tokens
    let mut tokens: Vec<Token> = Vec::new();

    // Create an vec of chars to loop over the input
    let mut chars = input.chars();

    // We run an infinite loop. The loop exists when theres no more chars
    // left to parse.
    loop {
        // we store the current char in a variable
        let next = chars.next();
        if next.is_none() {
            break;
        }
        let mut c = next.unwrap().to_string();

        // The first thing we want to check for is an open parenthesis. This will
        // later be used for `CallExpressions` but for now we only care about the
        // character.
        //
        // We check to see if we have an open parenthesis:
        if c == "(" {
            // If we do, we append a new token to our slice with the kind `paren`
            // and set the value to an open parenthesis.
            tokens.push(Token {
                kind: TokenKind::Paren,
                value: "(".to_string(),
            });

            // and `continue` to our next iteration
            continue;
        }

        // Next we're going to check for a closing parenthesis. We do the same exact
        // thing as before: Check for a closing parenthesis, add a new token,
        // and `continue`.
        if c == ")" {
            tokens.push(Token {
                kind: TokenKind::Paren,
                value: ")".to_string(),
            });
            continue;
        }

        // Moving on, we're now going to check for whitespace. This is interesting
        // because we care that whitespace exists to separate characters, but it
        // isn't actually important for us to store as a token. We would only throw
        // it out later.
        //
        // So here we're just going to test for existence and if it does exist we're
        // going to just `continue` on.
        if c.is_empty() {
            continue;
        }

        // The next type of token is a number. This is different than what we have
        // seen before because a number could be any number of characters and we
        // want to capture the entire sequence of characters as one token.
        //
        //   (add 123 456)
        //        ^^^ ^^^
        //        Only two separate tokens
        //
        // So we start this off when we encounter the first number in a sequence.

        // We first write a small closure to check wether an str is numeric or not
        let is_numeric = |x: &str| x.chars().all(char::is_numeric);

        if is_numeric(&c) {
            // We're going to create a `value` string that we are going to append
            // characters to.
            let mut value = String::new();

            // Then we're going to loop through each character in the sequence until
            // we encounter a character that is not a number, pushing each character
            // that is a number to our `value` and incrementing `current` as we go.
            while is_numeric(&c) {
                value.push_str(&c);
                // current += 1;
                let next = chars.next();
                if next.is_none() {
                    break;
                }
                c = next.unwrap().to_string();
            }

            // After that we append our `number` token to the `tokens` slice.
            tokens.push(Token {
                kind: TokenKind::Number,
                value,
            });

            continue;
        }

        // The last type of token will be a `name` token. This is a sequence of
        // letters instead of numbers, that are the names of functions in our lisp
        // syntax.
        //
        //   (add 2 4)
        //    ^^^
        //    Name token
        //

        // Again we define a small closure to determine wether a string is alphabet or not
        let is_alpha = |x: &str| x.chars().all(|z| z.is_ascii_alphabetic());
        if is_alpha(&c) {
            let mut value = String::new();

            while is_alpha(&c) {
                value.push_str(&c);
                c = chars.next().unwrap().to_string();
            }

            tokens.push(Token {
                kind: TokenKind::Name,
                value,
            });
            continue;
        }

        // Just in case, we print out any other type of string we received for debugging purposes
        eprintln!("{c}");
        break;
    }

    // Then at the end of our `tokenizer` we simply return the tokens vec.
    tokens
}

fn main() {
    let code = "(add 10 (subtract 10 6))";
    let tokens = tokenizer(code.to_string());

    println!("{tokens:#?}");
}
