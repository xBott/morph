package packets;

import me.bottdev.morph.runtime.MorphPacket;
import me.bottdev.morph.runtime.BinaryWriter;
import me.bottdev.morph.runtime.BinaryReader;
import me.bottdev.morph.runtime.PacketRegistries;

import java.io.ByteArrayOutputStream;
import java.io.InputStream;
import java.io.IOException;
import java.util.Objects;

public final class Particle implements MorphPacket {

	public static final byte PACKET_ID = 88;

	private ParticleOptions options;
	private int count;

	public Particle(
		ParticleOptions options,
		int count
	) {
		this.options = options;
		this.count = count;
	}

	public Particle() {}

	@Override
	public byte getPacketId() {{
		return PACKET_ID;
	}}

	public ParticleOptions getOptions() {
		return options;
	}

	public int getCount() {
		return count;
	}

	public void setOptions(ParticleOptions value) {
		this.options = value;
	}

	public void setCount(int value) {
		this.count = value;
	}

	@Override
	public void encode(ByteArrayOutputStream out, boolean encodeId) {

		if (encodeId) BinaryWriter.writeByte(out, PACKET_ID);

		options.encode(out, false);

		BinaryWriter.writeInt(out, count);

	}
	public static Particle decode(InputStream in) throws IOException {
		try {
			ParticleOptions options = ParticleOptions.decode(in);

			int count = BinaryReader.readInt(in);

			return new Particle(
				options,
				count
			);

		} catch (Exception e) {
			throw new IOException("Failed to decode Particle", e);

		}
	}

	static {
		PacketRegistries.DEFAULT.register(PACKET_ID, Particle::decode);
	}

	@Override
	public String toString() {
		return "Particle{" +
			"options=" + options +
			", " + "count=" + count +
		'}';
	}

	@Override
	public boolean equals(Object o) {
		if (this == o) return true;
		if (o == null || getClass() != o.getClass()) return false;
		Particle that = (Particle) o;
		return Objects.equals(options, that.options)
			&& count == that.count;
	}

	@Override
	public int hashCode() {
		return Objects.hash(options, count);
	}

}