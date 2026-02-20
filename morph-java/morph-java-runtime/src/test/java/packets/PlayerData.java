package packets;

import me.bottdev.morph.runtime.MorphPacket;
import me.bottdev.morph.runtime.BinaryWriter;
import me.bottdev.morph.runtime.BinaryReader;
import me.bottdev.morph.runtime.PacketRegistries;

import java.io.ByteArrayOutputStream;
import java.io.InputStream;
import java.io.IOException;
import java.util.Objects;

public final class PlayerData implements MorphPacket {

	public static final byte PACKET_ID = 27;

	private String name;
	private String lang;
	private long lastOnline;
	private Position position;
	private boolean dead;

	public PlayerData(
		String name,
		String lang,
		long lastOnline,
		Position position,
		boolean dead
	) {
		this.name = name;
		this.lang = lang;
		this.lastOnline = lastOnline;
		this.position = position;
		this.dead = dead;
	}

	public PlayerData() {}

	@Override
	public byte getPacketId() {{
		return PACKET_ID;
	}}

	public String getName() {
		return name;
	}

	public String getLang() {
		return lang;
	}

	public long getLastOnline() {
		return lastOnline;
	}

	public Position getPosition() {
		return position;
	}

	public boolean isDead() {
		return dead;
	}

	public void setName(String value) {
		this.name = value;
	}

	public void setLang(String value) {
		this.lang = value;
	}

	public void setLastOnline(long value) {
		this.lastOnline = value;
	}

	public void setPosition(Position value) {
		this.position = value;
	}

	public void setDead(boolean value) {
		this.dead = value;
	}

	@Override
	public void encode(ByteArrayOutputStream out, boolean encodeId) {

		if (encodeId) BinaryWriter.writeByte(out, PACKET_ID);

		BinaryWriter.writeString(out, name);

		BinaryWriter.writeString(out, lang);

		BinaryWriter.writeLong(out, lastOnline);

		position.encode(out, false);

		BinaryWriter.writeBoolean(out, dead);

	}
	public static PlayerData decode(InputStream in) throws IOException {
		try {
			String name = BinaryReader.readString(in);

			String lang = BinaryReader.readString(in);

			long last_online = BinaryReader.readLong(in);

			Position position = Position.decode(in);

			boolean dead = BinaryReader.readBoolean(in);

			return new PlayerData(
				name,
				lang,
				last_online,
				position,
				dead
			);

		} catch (Exception e) {
			throw new IOException("Failed to decode PlayerData", e);

		}
	}

	static {
		PacketRegistries.DEFAULT.register(PACKET_ID, PlayerData::decode);
	}

	@Override
	public String toString() {
		return "PlayerData{" +
			"name=" + name +
			", " + "lang=" + lang +
			", " + "lastOnline=" + lastOnline +
			", " + "position=" + position +
			", " + "dead=" + dead +
		'}';
	}

	@Override
	public boolean equals(Object o) {
		if (this == o) return true;
		if (o == null || getClass() != o.getClass()) return false;
		PlayerData that = (PlayerData) o;
		return Objects.equals(name, that.name)
			&& Objects.equals(lang, that.lang)
			&& lastOnline == that.lastOnline
			&& Objects.equals(position, that.position)
			&& dead == that.dead;
	}

	@Override
	public int hashCode() {
		return Objects.hash(name, lang, lastOnline, position, dead);
	}

}