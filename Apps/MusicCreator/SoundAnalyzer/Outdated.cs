using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace SoundAnalyzer
{
    class Outdated
    {
        //private void PlotNaudioWave(string path)
        //{
        //    chart1.Series[0].Points.Clear();
        //    chart1.Series[1].Points.Clear();
        //    chart1.Series[2].Points.Clear();
        //    chart1.ChartAreas[2].AxisX.MinorTickMark.Enabled = true;

        //    NAudio.Wave.WaveChannel32 wave = new NAudio.Wave.WaveChannel32(new NAudio.Wave.WaveFileReader(path));
        //    int size = 4096 * 4;
        //    byte[] buffer = new byte[size];
        //    List<Complex> complexs = new List<Complex>();
        //    int read = 0;

        //    while (wave.Position < size * 25)
        //    {
        //        read = wave.Read(buffer, 0, size);
        //        for (int i = 0; i < read / 4; i++)
        //        {
        //            float value = BitConverter.ToSingle(buffer, i * 4);
        //            chart1.Series[0].Points.Add(value);
        //            complexs.Add(new Complex(value, 0));
        //        }
        //    }

        //    Complex[] complexArray = complexs.ToArray();
        //    Fourier.Forward(complexArray, FourierOptions.NoScaling);
        //    for (int i = 0; i < 100; i++)
        //        chart1.Series[2].Points.Add(complexArray[i].Magnitude);
        //}

        //private void SinWave(int frequence)
        //{
        //    int numSample = 1000;
        //    chart1.Series[0].Points.Clear();

        //    double[] fundamental = Generate.Sinusoidal(numSample, 2000, frequence, 10);
        //    double[] fundamental2 = Generate.Sinusoidal(numSample, 2000, frequence * 5, 5);
        //    double[] fundamental3 = Generate.Sinusoidal(numSample, 2000, frequence * 3, 1);

        //    Complex[] complex32s = new Complex[numSample];
        //    for (int i = 0; i < numSample / 5; i++)
        //    {
        //        double time = ((i + 1) / numSample) * 0.5;
        //        chart1.Series[0].Points.AddXY(time, fundamental[i] + fundamental2[i] + fundamental3[i]);
        //        complex32s[i] = new Complex((float)fundamental[i] + fundamental2[i] + fundamental3[i], 0);
        //    }

        //    Fourier.Forward(complex32s, FourierOptions.Default);
        //    chart1.Series[2].Points.Clear();
        //    chart1.ChartAreas[2].AxisX.MinorTickMark.Enabled = true;

        //    for (int i = 1; i < complex32s.Length / 2.2; i++)
        //        chart1.Series[2].Points.AddXY((2000 / numSample) * i, complex32s[i].Magnitude);

        //    Fourier.Inverse(complex32s);
        //    for (int i = 0; i < numSample / 5; i++)
        //        chart1.Series[1].Points.AddXY(((i + 1) / numSample) * 0.5, complex32s[i].Real);
        //}
    }
}
