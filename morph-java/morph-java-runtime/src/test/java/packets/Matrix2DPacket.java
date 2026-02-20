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

public final class Matrix2DPacket implements MorphPacket {

	public static final byte PACKET_ID = 22;

	private List<List<Integer>> data = new ArrayList<>();

	public Matrix2DPacket(
		List<List<Integer>> data
	) {
		this.data = data;
	}

	public Matrix2DPacket() {}

	@Override
	public byte getPacketId() {{
		return PACKET_ID;
	}}

	public List<List<Integer>> getData() {
		return data;
	}

	public void setData(List<List<Integer>> value) {
		this.data = value;
	}

	@Override
	public void encode(ByteArrayOutputStream out, boolean encodeId) {

		if (encodeId) BinaryWriter.writeByte(out, PACKET_ID);

		BinaryWriter.writeInt(out, data.size());
		for (List<Integer> dataItem : data) {

			BinaryWriter.writeInt(out, dataItem.size());
			for (int dataItemItem1 : dataItem) {

				BinaryWriter.writeInt(out, dataItemItem1);

			}

		}

	}
	public static Matrix2DPacket decode(InputStream in) throws IOException {
		try {
			int dataLength = BinaryReader.readInt(in);
			ArrayList<List<Integer>> data = new ArrayList<>();
			for (int dataIndex = 0; dataIndex < dataLength; dataIndex++) {
				int dataItemLength = BinaryReader.readInt(in);
				ArrayList<Integer> data_item = new ArrayList<>();
				for (int dataItemIndex = 0; dataItemIndex < dataItemLength; dataItemIndex++) {
					int data_item_item = BinaryReader.readInt(in);

					data_item.add(data_item_item);
				}

				data.add(data_item);
			}

			return new Matrix2DPacket(
				data
			);

		} catch (Exception e) {
			throw new IOException("Failed to decode Matrix2DPacket", e);

		}
	}

	static {
		PacketRegistries.DEFAULT.register(PACKET_ID, Matrix2DPacket::decode);
	}

	@Override
	public String toString() {
		return "Matrix2DPacket{" +
			"data=" + data +
		'}';
	}

	@Override
	public boolean equals(Object o) {
		if (this == o) return true;
		if (o == null || getClass() != o.getClass()) return false;
		Matrix2DPacket that = (Matrix2DPacket) o;
		return Objects.equals(data, that.data);
	}

	@Override
	public int hashCode() {
		return Objects.hash(data);
	}

}