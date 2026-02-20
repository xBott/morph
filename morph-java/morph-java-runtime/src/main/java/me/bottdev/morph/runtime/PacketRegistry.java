package me.bottdev.morph.runtime;

import java.io.IOException;
import java.io.InputStream;
import java.util.HashMap;
import java.util.Optional;

public class PacketRegistry {

    @FunctionalInterface
    public interface Decoder {
        MorphPacket decode(InputStream in) throws IOException;
    }

    private final HashMap<Byte, Decoder> decoders = new HashMap<>();

    public void register(byte id, Decoder decoder) {
        decoders.put(id, decoder);
    }

    public Decoder unregister(byte id) {
        return decoders.remove(id);
    }

    public void unregisterAll() {
        decoders.clear();
    }

    public Optional<Decoder> find(byte id) {
        return Optional.ofNullable(decoders.get(id));
    }

}
