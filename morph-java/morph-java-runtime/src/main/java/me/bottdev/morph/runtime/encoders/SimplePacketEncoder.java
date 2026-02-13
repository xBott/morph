package me.bottdev.morph.runtime.encoders;

import me.bottdev.morph.runtime.MorphPacket;
import me.bottdev.morph.runtime.PacketEncoder;

public class SimplePacketEncoder implements PacketEncoder {

    @Override
    public byte[] encode(MorphPacket packet) {
        return packet.encode();
    }

}
