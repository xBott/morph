package packets;

import me.bottdev.morph.runtime.MorphPacket;
import me.bottdev.morph.runtime.BinaryWriter;
import me.bottdev.morph.runtime.BinaryReader;
import me.bottdev.morph.runtime.PacketRegistries;

import java.io.ByteArrayOutputStream;
import java.io.InputStream;
import java.io.IOException;
import java.util.Objects;

public final class ParticleOptions implements MorphPacket {

	public static final byte PACKET_ID = 117;

	private String type;
	private float speed;

	public ParticleOptions(
		String type,
		float speed
	) {
		this.type = type;
		this.speed = speed;
	}

	public ParticleOptions() {}

	@Override
	public byte getPacketId() {{
		return PACKET_ID;
	}}

	public String getType() {
		return type;
	}

	public float getSpeed() {
		return speed;
	}

	public void setType(String value) {
		this.type = value;
	}

	public void setSpeed(float value) {
		this.speed = value;
	}

	@Override
	public void encode(ByteArrayOutputStream out, boolean encodeId) {

		if (encodeId) BinaryWriter.writeByte(out, PACKET_ID);

		BinaryWriter.writeString(out, type);

		BinaryWriter.writeFloat(out, speed);

	}
	public static ParticleOptions decode(InputStream in) throws IOException {
		try {
			String type = BinaryReader.readString(in);

			float speed = BinaryReader.readFloat(in);

			return new ParticleOptions(
				type,
				speed
			);

		} catch (Exception e) {
			throw new IOException("Failed to decode ParticleOptions", e);

		}
	}

	static {
		PacketRegistries.DEFAULT.register(PACKET_ID, ParticleOptions::decode);
	}

	@Override
	public String toString() {
		return "ParticleOptions{" +
			"type=" + type +
			", " + "speed=" + speed +
		'}';
	}

	@Override
	public boolean equals(Object o) {
		if (this == o) return true;
		if (o == null || getClass() != o.getClass()) return false;
		ParticleOptions that = (ParticleOptions) o;
		return Objects.equals(type, that.type)
			&& speed == that.speed;
	}

	@Override
	public int hashCode() {
		return Objects.hash(type, speed);
	}

}