package me.bottdev.morph.runtime;

import me.bottdev.morph.runtime.exceptions.MorphDecodingException;

import java.io.BufferedInputStream;
import java.io.ByteArrayInputStream;
import java.io.InputStream;
import java.util.Optional;

public interface PacketDecoder {

    MorphPacket decodeStream(InputStream in) throws MorphDecodingException;

    default MorphPacket decodeData(byte[] data) {
        InputStream in = new BufferedInputStream(new ByteArrayInputStream(data));
        return decodeStream(in);
    }

    default Optional<MorphPacket> decodeStreamSafe(InputStream in) {
        try {
            MorphPacket packet = decodeStream(in);
            return Optional.of(packet);

        } catch (MorphDecodingException ex) {
            return Optional.empty();
        }
    }

    default Optional<MorphPacket> decodeDataSafe(byte[] data) {
        try {
            MorphPacket packet = decodeData(data);
            return Optional.of(packet);

        } catch (MorphDecodingException ex) {
            return Optional.empty();
        }
    }

}
