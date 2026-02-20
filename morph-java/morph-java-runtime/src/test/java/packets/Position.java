package packets;

import me.bottdev.morph.runtime.MorphPacket;
import me.bottdev.morph.runtime.BinaryWriter;
import me.bottdev.morph.runtime.BinaryReader;
import me.bottdev.morph.runtime.PacketRegistries;

import java.io.ByteArrayOutputStream;
import java.io.InputStream;
import java.io.IOException;
import java.util.Objects;

public final class Position implements MorphPacket {

	public static final byte PACKET_ID = 109;

	private float x;
	private float y;
	private float z;
	private float yaw;
	private float pitch;
	private String world;

	public Position(
		float x,
		float y,
		float z,
		float yaw,
		float pitch,
		String world
	) {
		this.x = x;
		this.y = y;
		this.z = z;
		this.yaw = yaw;
		this.pitch = pitch;
		this.world = world;
	}

	public Position() {}

	@Override
	public byte getPacketId() {{
		return PACKET_ID;
	}}

	public float getX() {
		return x;
	}

	public float getY() {
		return y;
	}

	public float getZ() {
		return z;
	}

	public float getYaw() {
		return yaw;
	}

	public float getPitch() {
		return pitch;
	}

	public String getWorld() {
		return world;
	}

	public void setX(float value) {
		this.x = value;
	}

	public void setY(float value) {
		this.y = value;
	}

	public void setZ(float value) {
		this.z = value;
	}

	public void setYaw(float value) {
		this.yaw = value;
	}

	public void setPitch(float value) {
		this.pitch = value;
	}

	public void setWorld(String value) {
		this.world = value;
	}

	@Override
	public void encode(ByteArrayOutputStream out, boolean encodeId) {

		if (encodeId) BinaryWriter.writeByte(out, PACKET_ID);

		BinaryWriter.writeFloat(out, x);

		BinaryWriter.writeFloat(out, y);

		BinaryWriter.writeFloat(out, z);

		BinaryWriter.writeFloat(out, yaw);

		BinaryWriter.writeFloat(out, pitch);

		BinaryWriter.writeString(out, world);

	}
	public static Position decode(InputStream in) throws IOException {
		try {
			float x = BinaryReader.readFloat(in);

			float y = BinaryReader.readFloat(in);

			float z = BinaryReader.readFloat(in);

			float yaw = BinaryReader.readFloat(in);

			float pitch = BinaryReader.readFloat(in);

			String world = BinaryReader.readString(in);

			return new Position(
				x,
				y,
				z,
				yaw,
				pitch,
				world
			);

		} catch (Exception e) {
			throw new IOException("Failed to decode Position", e);

		}
	}

	static {
		PacketRegistries.DEFAULT.register(PACKET_ID, Position::decode);
	}

	@Override
	public String toString() {
		return "Position{" +
			"x=" + x +
			", " + "y=" + y +
			", " + "z=" + z +
			", " + "yaw=" + yaw +
			", " + "pitch=" + pitch +
			", " + "world=" + world +
		'}';
	}

	@Override
	public boolean equals(Object o) {
		if (this == o) return true;
		if (o == null || getClass() != o.getClass()) return false;
		Position that = (Position) o;
		return x == that.x
			&& y == that.y
			&& z == that.z
			&& yaw == that.yaw
			&& pitch == that.pitch
			&& Objects.equals(world, that.world);
	}

	@Override
	public int hashCode() {
		return Objects.hash(x, y, z, yaw, pitch, world);
	}

}