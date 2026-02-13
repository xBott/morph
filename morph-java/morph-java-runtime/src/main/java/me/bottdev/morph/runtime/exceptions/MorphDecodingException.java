package me.bottdev.morph.runtime.exceptions;

public class MorphDecodingException extends RuntimeException {
    public MorphDecodingException(String message) {
        super(message);
    }

    public MorphDecodingException(String message, Throwable cause) {
      super(message, cause);
    }

}
