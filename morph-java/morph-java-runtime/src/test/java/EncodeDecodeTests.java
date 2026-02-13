import me.bottdev.morph.runtime.MorphPacket;
import me.bottdev.morph.runtime.PacketDecoder;
import me.bottdev.morph.runtime.PacketEncoder;
import me.bottdev.morph.runtime.PacketRegistries;
import me.bottdev.morph.runtime.decoders.SimplePacketDecoder;
import me.bottdev.morph.runtime.encoders.SimplePacketEncoder;
import packets.AuthRequest;
import org.junit.jupiter.api.Assertions;
import org.junit.jupiter.api.BeforeAll;
import org.junit.jupiter.api.Test;
import packets.AuthResponse;

public class EncodeDecodeTests {

    static PacketEncoder packetEncoder;
    static PacketDecoder packetDecoder;

    @BeforeAll
    public static void setup() {
        packetEncoder = new SimplePacketEncoder();
        packetDecoder = new SimplePacketDecoder(PacketRegistries.DEFAULT);
    }

    @Test
    public void testEncodeDecodeAuthRequest() {

        MorphPacket packet = new AuthRequest("admin", "admin12345");
        String initialStr = packet.toString();

        byte[] data = packetEncoder.encode(packet);

        Assertions.assertTrue(data.length > 0);

        MorphPacket decoded = packetDecoder.decode(data);
        String finalStr = decoded.toString();

        Assertions.assertEquals(AuthRequest.class, decoded.getClass());
        Assertions.assertEquals(initialStr, finalStr);

    }

    @Test
    public void testEncodeDecodeAuthResponse() {

        MorphPacket packet = new AuthResponse(true, "gk19gasdi9gretjas");
        String initialStr = packet.toString();

        byte[] data = packetEncoder.encode(packet);

        Assertions.assertTrue(data.length > 0);

        MorphPacket decoded = packetDecoder.decode(data);
        String finalStr = decoded.toString();

        Assertions.assertEquals(AuthResponse.class, decoded.getClass());
        Assertions.assertEquals(initialStr, finalStr);

    }

}
