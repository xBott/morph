package me.bottdev.morph.runtime;

import java.io.ByteArrayOutputStream;
import java.nio.charset.StandardCharsets;

public final class BinaryWriter {

    private BinaryWriter() {}

    public static void writeByte(ByteArrayOutputStream out, byte value) {
        out.write(value & 0xFF);
    }

    public static void writeBoolean(ByteArrayOutputStream out, boolean value) {
        out.write(value ? 1 : 0);
    }

    public static void writeShort(ByteArrayOutputStream out, short value) {
        out.write((value >>> 8) & 0xFF);
        out.write(value & 0xFF);
    }

    public static void writeInt(ByteArrayOutputStream out, int value) {
        out.write((value >>> 24) & 0xFF);
        out.write((value >>> 16) & 0xFF);
        out.write((value >>> 8) & 0xFF);
        out.write(value & 0xFF);
    }

    public static void writeLong(ByteArrayOutputStream out, long value) {
        out.write((int)(value >>> 56) & 0xFF);
        out.write((int)(value >>> 48) & 0xFF);
        out.write((int)(value >>> 40) & 0xFF);
        out.write((int)(value >>> 32) & 0xFF);
        out.write((int)(value >>> 24) & 0xFF);
        out.write((int)(value >>> 16) & 0xFF);
        out.write((int)(value >>> 8) & 0xFF);
        out.write((int)value & 0xFF);
    }

    public static void writeFloat(ByteArrayOutputStream out, float value) {
        writeInt(out, Float.floatToIntBits(value));
    }

    public static void writeDouble(ByteArrayOutputStream out, double value) {
        writeLong(out, Double.doubleToLongBits(value));
    }

    public static void writeChar(ByteArrayOutputStream out, char value) {
        writeShort(out, (short) value);
    }

    public static void writeString(ByteArrayOutputStream out, String value) {
        byte[] bytes = value.getBytes(StandardCharsets.UTF_8);
        writeInt(out, bytes.length);
        out.write(bytes, 0, bytes.length);
    }
    
    public static void writeBytes(ByteArrayOutputStream out, byte[] value) {
        out.writeBytes(value);
    }

}
