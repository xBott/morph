package me.bottdev.morph.runtime;

import java.util.HashMap;
import java.util.Optional;

public class PacketRegistry {

    @FunctionalInterface
    public interface Decoder {
        MorphPacket decode(byte[] bytes);
    }

    private final HashMap<Integer, Decoder> decoders = new HashMap<>();

    public void register(int id, Decoder decoder) {
        decoders.put(id, decoder);
    }

    public Decoder unregister(int id) {
        return decoders.remove(id);
    }

    public void unregisterAll() {
        decoders.clear();
    }

    public Optional<Decoder> find(int id) {
        return Optional.ofNullable(decoders.get(id));
    }

}
