package me.bottdev.morph.runtime;

import java.io.IOException;
import java.io.InputStream;
import java.nio.charset.StandardCharsets;

public final class BinaryReader {

    private BinaryReader() {}

    public static byte readByte(InputStream in) throws IOException {
        int val = in.read();
        if (val == -1) throw new IOException("Unexpected EOF while reading byte");
        return (byte) val;
    }

    public static boolean readBoolean(InputStream in) throws IOException {
        int val = in.read();
        if (val == -1) throw new IOException("Unexpected EOF while reading boolean");
        return val != 0;
    }

    public static short readShort(InputStream in) throws IOException {
        int high = in.read();
        int low  = in.read();
        if ((high | low) < 0) throw new IOException("Unexpected EOF while reading short");
        return (short)((high << 8) | low);
    }

    public static char readChar(InputStream in) throws IOException {
        return (char) readShort(in);
    }

    public static int readInt(InputStream in) throws IOException {
        int b1 = in.read();
        int b2 = in.read();
        int b3 = in.read();
        int b4 = in.read();
        if ((b1 | b2 | b3 | b4) < 0) throw new IOException("Unexpected EOF while reading int");
        return (b1 << 24) | (b2 << 16) | (b3 << 8) | b4;
    }

    public static long readLong(InputStream in) throws IOException {
        long b1 = in.read();
        long b2 = in.read();
        long b3 = in.read();
        long b4 = in.read();
        long b5 = in.read();
        long b6 = in.read();
        long b7 = in.read();
        long b8 = in.read();
        if ((b1 | b2 | b3 | b4 | b5 | b6 | b7 | b8) < 0)
            throw new IOException("Unexpected EOF while reading long");
        return (b1 << 56) | (b2 << 48) | (b3 << 40) | (b4 << 32) |
                (b5 << 24) | (b6 << 16) | (b7 << 8) | b8;
    }

    public static float readFloat(InputStream in) throws IOException {
        return Float.intBitsToFloat(readInt(in));
    }

    public static double readDouble(InputStream in) throws IOException {
        return Double.longBitsToDouble(readLong(in));
    }

    public static String readString(InputStream in) throws IOException {
        int length = readInt(in);
        if (length < 0) throw new IOException("Negative string length");
        byte[] bytes = in.readNBytes(length);
        if (bytes.length != length) throw new IOException("Unexpected EOF while reading string");
        return new String(bytes, StandardCharsets.UTF_8);
    }

}
