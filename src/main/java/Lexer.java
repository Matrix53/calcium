import java.util.Map;

public class Lexer {
    private Map<String, String> map = Map.ofEntries(
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

    public static void main(String[] args) {

    }
}
