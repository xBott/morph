import me.bottdev.morph.runtime.MorphPacket;
import me.bottdev.morph.runtime.PacketDecoder;
import me.bottdev.morph.runtime.PacketEncoder;
import me.bottdev.morph.runtime.PacketRegistries;
import me.bottdev.morph.runtime.decoders.SimplePacketDecoder;
import me.bottdev.morph.runtime.encoders.SimplePacketEncoder;
import org.junit.jupiter.api.Assertions;
import org.junit.jupiter.api.BeforeAll;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.ValueSource;
import packets.*;

import java.util.ArrayList;
import java.util.List;

public class EncodeDecodeTests {

    static PacketEncoder packetEncoder;
    static PacketDecoder packetDecoder;

    @BeforeAll
    public static void setup() {
        packetEncoder = new SimplePacketEncoder();
        packetDecoder = new SimplePacketDecoder(PacketRegistries.DEFAULT);
    }

    @Test
    public void testEncodeDecodePosition() {

        Position position = new Position();
        position.setX(10f);
        position.setY(20f);
        position.setZ(30f);
        position.setYaw(0f);
        position.setPitch(0f);
        position.setWorld("world");
        String initialStr = position.toString();

        byte[] data = packetEncoder.encode(position);

        Assertions.assertTrue(data.length > 0);

        MorphPacket decoded = packetDecoder.decodeData(data);
        String finalStr = decoded.toString();

        Assertions.assertEquals(Position.class, decoded.getClass());
        Assertions.assertEquals(initialStr, finalStr);

        System.out.println(finalStr);

    }

    @Test
    public void testEncodeDecodePlayerData() {

        Position position = new Position();
        position.setX(10f);
        position.setY(20f);
        position.setZ(30f);
        position.setYaw(0f);
        position.setPitch(0f);
        position.setWorld("world");

        PlayerData playerData = new PlayerData();
        playerData.setPosition(position);
        playerData.setDead(false);
        playerData.setLang("en");
        playerData.setName("player1");
        playerData.setLastOnline(System.currentTimeMillis());

        String initialStr = playerData.toString();

        byte[] data = packetEncoder.encode(playerData);

        Assertions.assertTrue(data.length > 0);

        MorphPacket decoded = packetDecoder.decodeData(data);
        String finalStr = decoded.toString();

        Assertions.assertEquals(PlayerData.class, decoded.getClass());
        Assertions.assertEquals(initialStr, finalStr);

        System.out.println(finalStr);

    }

    @Test
    public void testEncodeDecodeParticles() {

        Particles particles = new Particles();
        String[] particleTypes = new String[] {"flame", "smoke", "lightning"};

        for (int i = 0; i < 10; i++) {
            ParticleOptions options = new ParticleOptions();

            int index = (i + 1) % particleTypes.length;
            options.setType(particleTypes[index]);

            options.setSpeed(i * 0.5f);

            Particle particle = new Particle();
            particle.setOptions(options);
            particle.setCount(10);

            particles.getParticles().add(particle);
        }

        String initialStr = particles.toString();

        byte[] data = packetEncoder.encode(particles);

        Assertions.assertTrue(data.length > 0);

        MorphPacket decoded = packetDecoder.decodeData(data);
        String finalStr = decoded.toString();

        Assertions.assertEquals(Particles.class, decoded.getClass());
        Assertions.assertEquals(initialStr, finalStr);

        System.out.println(finalStr);

    }

    @ParameterizedTest
    @ValueSource(ints = {3, 5, 7, 10, 20})
    public void testEncodeDecodeMatrix2D(int input) {

        Matrix2DPacket matrix = new Matrix2DPacket();

        for (int row = 0; row < input; row++) {

            List<Integer> rowList = new ArrayList<>();

            for (int col = 0; col < input; col++) {
                rowList.add(col + row);
            }

            matrix.getData().add(rowList);

        }

        String initialStr = matrix.toString();
        byte[] data = packetEncoder.encode(matrix);
        System.out.println("Length of packet with size " + input + ": " + data.length);

        Assertions.assertTrue(data.length > 0);

        MorphPacket decoded = packetDecoder.decodeData(data);
        String finalStr = decoded.toString();

        Assertions.assertEquals(Matrix2DPacket.class, decoded.getClass());
        Assertions.assertEquals(initialStr, finalStr);

        System.out.println(finalStr);

    }

}
