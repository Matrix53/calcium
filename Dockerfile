FROM openjdk:16.0.2
COPY . /lexer/
WORKDIR /lexer/
RUN javac -cp src/main/java src/main/java/Lexer.java -d dst/