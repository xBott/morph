package me.bottdev.morph.runtime;

import java.io.ByteArrayOutputStream;

public interface MorphPacket {

    byte getPacketId();

    void encode(ByteArrayOutputStream out, boolean encodeId);

}
