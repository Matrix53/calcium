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
                res = '\u0000';
            } else {
                res = (char) tmp;
            }
        }
        return res;
    }

    private void ungetChar(char src) {
        hasBuffer = true;
        buffer = src;
    }

    public String nextToken() throws IOException {
        if (!hasNext) return null;
        String token = "";
        char ch = getChar();

        token += ch;
        return token;
    }

    public static void main(String[] args) throws IOException {
        Lexer lexer = new Lexer(System.in);
        while (lexer.hasNext()) System.out.println(lexer.nextToken());
    }
}
