package me.bottdev.morph.runtime.encoders;

import me.bottdev.morph.runtime.MorphPacket;
import me.bottdev.morph.runtime.PacketEncoder;

import java.io.ByteArrayOutputStream;

public class SimplePacketEncoder implements PacketEncoder {

    @Override
    public byte[] encode(MorphPacket packet) {
        ByteArrayOutputStream out = new ByteArrayOutputStream();
        packet.encode(out, true);
        return out.toByteArray();
    }

}
