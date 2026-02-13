package me.bottdev.morph.runtime.decoders;

import me.bottdev.morph.runtime.MorphPacket;
import me.bottdev.morph.runtime.PacketDecoder;
import me.bottdev.morph.runtime.PacketRegistry;
import me.bottdev.morph.runtime.exceptions.MorphDecodingException;

import java.nio.ByteBuffer;
import java.util.Optional;

public class SimplePacketDecoder implements PacketDecoder {

    private final PacketRegistry registry;

    public SimplePacketDecoder(PacketRegistry registry) {
        this.registry = registry;
    }

    private int extractPacketId(byte[] data) throws IllegalArgumentException {

        if (data == null || data.length < 4) {
            throw new IllegalArgumentException("Data too short to contain packet id");
        }

        ByteBuffer buffer = ByteBuffer.wrap(data);
        return buffer.getInt();

    }

    @Override
    public MorphPacket decode(byte[] data) throws MorphDecodingException {

        try {

            int id = extractPacketId(data);

            Optional<PacketRegistry.Decoder> decoderOptional = registry.find(id);
            if (decoderOptional.isEmpty()) return null;

            PacketRegistry.Decoder decoder = decoderOptional.get();
            return decoder.decode(data);

        } catch (IllegalArgumentException ex) {
            throw new MorphDecodingException("Could not decode data", ex);
        }

    }

    @Override
    public Optional<MorphPacket> decodeSafe(byte[] data) {
        try {
            MorphPacket packet = decode(data);
            return Optional.of(packet);

        } catch (MorphDecodingException ex) {
            return Optional.empty();
        }
    }

}
