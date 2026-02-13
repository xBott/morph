package me.bottdev.morph.runtime;

public interface MorphPacket {

    int getPacketId();

    byte[] encode();

}
