package me.bottdev.morph.runtime;

public interface PacketEncoder {

    byte[] encode(MorphPacket packet);

}
