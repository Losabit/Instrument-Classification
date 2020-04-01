using NAudio.Wave;
using System;
using System.IO;
using System.Numerics;

namespace AudioLibrary
{
    public static class Common
    {
        public static FileInfo[] GetAllWavFiles(string directoryPath)
        {
            DirectoryInfo d = new DirectoryInfo(directoryPath);
            return d.GetFiles("*.wav");
        }

        public static void ImportAllNotes(string inPath, string outPath)
        {
            DirectoryInfo d = new DirectoryInfo(inPath);
            FileInfo[] Files = d.GetFiles("*.mp3");
            foreach (FileInfo file in Files)
            {
                ConvertMp3ToWav(inPath + @"\" + file.Name, outPath + @"\" + file.Name.Replace(".mp3", ".wav"));
            }
            Console.WriteLine("End of export");
        }

        public static void ConvertMp3ToWav(string _inPath_, string _outPath_)
        {
            using (Mp3FileReader mp3 = new Mp3FileReader(_inPath_))
            {
                using (WaveStream pcm = WaveFormatConversionStream.CreatePcmStream(mp3))
                {
                    WaveFileWriter.CreateWaveFile(_outPath_, pcm);
                }
            }
        }

        public static string BytesToString(byte[] bytes, int start, int length)
        {
            byte[] data = new byte[length];
            for (int i = 0; i < length; i++)
                data[i] = bytes[start + i];

            using (MemoryStream stream = new MemoryStream(data))
            {
                using (StreamReader streamReader = new StreamReader(stream))
                {
                    return streamReader.ReadToEnd();
                }
            }
        }

        public static T[] CutArray<T>(T[] bytes, int start, int length)
        {
            T[] data;
            if (length + start > bytes.Length)
                data = new T[bytes.Length - start];
            else
                data = new T[length];

            for (int i = 0; i < data.Length; i++)
                data[i] = bytes[start + i];

            return data;
        }

        public static ushort[] DataConverter8To16(byte[] data)
        {
            if (data.Length % 2 != 0)
                throw new Exception("Odd data can't be cast");

            ushort[] newData = new ushort[data.Length / 2];
            for (int i = 0; i < newData.Length; i++)
            {
                newData[i] = (ushort)((data[i * 2] << 8) | (data[i * 2 + 1] & 0xff));
            }
            return newData;
        }

        public static byte[] DataConverter16To8(ushort[] data)
        {
            byte[] newData = new byte[data.Length * 2];
            for (int i = 0; i < data.Length; i++)
            {
                byte[] value = BitConverter.GetBytes(data[i]);
                newData[i * 2] = value[1];
                newData[i * 2 + 1] = value[0];
            }
            return newData;
        }

        public static string[] GenerateFileName(string path, int size)
        {
            string[] paths = new string[size];
            for (int i = 0; i < size; i++)
                paths[i] = path.Insert(path.LastIndexOf("."), (i + 1).ToString());

            return paths;
        }
    }
}