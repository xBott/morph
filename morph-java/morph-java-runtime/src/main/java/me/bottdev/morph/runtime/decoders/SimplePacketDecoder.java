package me.bottdev.morph.runtime.decoders;

import me.bottdev.morph.runtime.BinaryReader;
import me.bottdev.morph.runtime.MorphPacket;
import me.bottdev.morph.runtime.PacketDecoder;
import me.bottdev.morph.runtime.PacketRegistry;
import me.bottdev.morph.runtime.exceptions.MorphDecodingException;

import java.io.IOException;
import java.io.InputStream;
import java.util.Optional;

public class SimplePacketDecoder implements PacketDecoder {

    private final PacketRegistry registry;

    public SimplePacketDecoder(PacketRegistry registry) {
        this.registry = registry;
    }

    @Override
    public MorphPacket decodeStream(InputStream in) throws MorphDecodingException {

        if (in == null) {
            throw new MorphDecodingException("Data too short to contain packet id");
        }

        byte id;
        try {
            id = BinaryReader.readByte(in);

        } catch (IOException ex) {
            throw new MorphDecodingException("Could not extract packet id", ex);
        }

        try {

            Optional<PacketRegistry.Decoder> decoderOptional = registry.find(id);
            if (decoderOptional.isEmpty()) return null;

            PacketRegistry.Decoder decoder = decoderOptional.get();
            return decoder.decode(in);

        } catch (IOException ex) {
            throw new MorphDecodingException("Could not decode data", ex);
        }

    }

}
