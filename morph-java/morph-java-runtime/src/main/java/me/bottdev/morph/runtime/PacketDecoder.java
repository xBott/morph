package me.bottdev.morph.runtime;

import java.util.Optional;

public interface PacketDecoder {

    MorphPacket decode(byte[] data);

    Optional<MorphPacket> decodeSafe(byte[] data);

}
