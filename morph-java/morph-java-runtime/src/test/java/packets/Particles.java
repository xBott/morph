package packets;

import me.bottdev.morph.runtime.MorphPacket;
import me.bottdev.morph.runtime.BinaryWriter;
import me.bottdev.morph.runtime.BinaryReader;
import me.bottdev.morph.runtime.PacketRegistries;

import java.io.ByteArrayOutputStream;
import java.io.InputStream;
import java.io.IOException;
import java.util.Objects;
import java.util.List;
import java.util.ArrayList;

public final class Particles implements MorphPacket {

	public static final byte PACKET_ID = 103;

	private List<Particle> particles = new ArrayList<>();

	public Particles(
		List<Particle> particles
	) {
		this.particles = particles;
	}

	public Particles() {}

	@Override
	public byte getPacketId() {{
		return PACKET_ID;
	}}

	public List<Particle> getParticles() {
		return particles;
	}

	public void setParticles(List<Particle> value) {
		this.particles = value;
	}

	@Override
	public void encode(ByteArrayOutputStream out, boolean encodeId) {

		if (encodeId) BinaryWriter.writeByte(out, PACKET_ID);

		BinaryWriter.writeInt(out, particles.size());
		for (Particle particlesItem : particles) {

			particlesItem.encode(out, false);

		}

	}
	public static Particles decode(InputStream in) throws IOException {
		try {
			int particlesLength = BinaryReader.readInt(in);
			ArrayList<Particle> particles = new ArrayList<>();
			for (int particlesIndex = 0; particlesIndex < particlesLength; particlesIndex++) {
				Particle particles_item = Particle.decode(in);

				particles.add(particles_item);
			}

			return new Particles(
				particles
			);

		} catch (Exception e) {
			throw new IOException("Failed to decode Particles", e);

		}
	}

	static {
		PacketRegistries.DEFAULT.register(PACKET_ID, Particles::decode);
	}

	@Override
	public String toString() {
		return "Particles{" +
			"particles=" + particles +
		'}';
	}

	@Override
	public boolean equals(Object o) {
		if (this == o) return true;
		if (o == null || getClass() != o.getClass()) return false;
		Particles that = (Particles) o;
		return Objects.equals(particles, that.particles);
	}

	@Override
	public int hashCode() {
		return Objects.hash(particles);
	}

}