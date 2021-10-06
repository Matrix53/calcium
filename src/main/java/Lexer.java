import java.io.IOException;
import java.io.InputStream;
import java.util.Map;

public class Lexer {
    private boolean hasNext;
    private boolean hasBuffer;
    private char buffer;
    private final InputStream in;
    private final Map<String, String> map = Map.ofEntries(
            Map.entry("if", "If"),
            Map.entry("else", "Else"),
            Map.entry("while", "While"),
            Map.entry("break", "Break"),
            Map.entry("continue", "Continue"),
            Map.entry("return", "Return"),
            Map.entry("=", "Assign"),
            Map.entry(";", "Semicolon"),
            Map.entry("(", "LPar"),
            Map.entry(")", "RPar"),
            Map.entry("{", "LBrace"),
            Map.entry("}", "RBrace"),
            Map.entry("+", "Plus"),
            Map.entry("*", "Mult"),
            Map.entry("/", "Div"),
            Map.entry("<", "Lt"),
            Map.entry(">", "Gt"),
            Map.entry("==", "Equal"));

    public Lexer(InputStream in) {
        this.in = in;
        this.hasNext = true;
    }

    public boolean hasNext() {
        return hasNext;
    }

    private char getChar() throws IOException {
        char res;
        if (hasBuffer) {
            res = buffer;
            hasBuffer = false;
        } else {
            int tmp = in.read();
            if (tmp == -1) {
                hasNext = false;
                res = ' ';
            } else {
                res = (char) tmp;
            }
        }
        return res;
    }

    private void ungetChar(char ch) {
        hasBuffer = true;
        buffer = ch;
        if (!Character.isWhitespace(ch)) hasNext = true;
    }

    private char getVisibleChar() throws IOException {
        char res = ' ';
        while (hasNext && Character.isWhitespace(res)) res = getChar();
        return res;
    }

    public String nextToken() throws IOException {
        // init
        if (!hasNext) return null;
        char ch = getVisibleChar();
        if (Character.isWhitespace(ch)) {
            hasNext = false;
            return null;
        }
        String token = String.valueOf(ch);

        // identifier
        if (Character.isLetter(ch) || ch == '_') {
            while (hasNext) {
                ch = getChar();
                if (Character.isLetterOrDigit(ch) || ch == '_') token += ch;
                else {
                    ungetChar(ch);
                    if (map.containsKey(token)) token = map.get(token);
                    else token = "Ident(" + token + ")";
                    break;
                }
            }
        }

        // number
        else if (Character.isDigit(ch)) {
            while (hasNext) {
                ch = getChar();
                if (Character.isDigit(ch)) token += ch;
                else {
                    ungetChar(ch);
                    token = "Number(" + token + ")";
                    break;
                }
            }
        }

        // equal
        else if (ch == '=') {
            ch = getChar();
            if (ch == '=') token = "Equal";
            else {
                ungetChar(ch);
                token = "Assign";
            }
        }

        // symbol
        else if (map.containsKey(token)) {
            token = map.get(token);
        }

        // error
        else {
            System.out.printf("\nErr");
            return null;
        }

        return token;
    }

    public static void main(String[] args) throws IOException {
        Lexer lexer = new Lexer(System.in);
        String firstToken = lexer.nextToken();
        if (firstToken != null) System.out.print(firstToken);
        else return;
        while (lexer.hasNext()) {
            String token = lexer.nextToken();
            if (token != null) System.out.printf("\n%s", token);
            else return;
        }
    }
}
