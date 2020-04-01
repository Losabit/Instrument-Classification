using System;
using System.Numerics;

namespace AudioLibrary
{
    public class FourierTransform
    {
        public static Complex[] Discrete(Complex[] complex)
        {
            Complex[] complex2 = new Complex[complex.Length];
            for (int k = 0; k < complex.Length; k++)
            {
                complex2[k] = new Complex(0, 0);
                for (int n = 0; n < complex.Length; n++)
                    complex2[k] += Complex.FromPolarCoordinates(1, -2 * Math.PI * n * k / complex.Length) * complex[n];
            }
            return complex2;
        }

        public static Complex[] Fast(Complex[] complex)
        {
            Complex[] complex2 = new Complex[complex.Length];
            if (complex.Length == 1)
            {
                complex2[0] = complex[0];
                return complex2;
            }

            Complex[] e = new Complex[complex.Length / 2];
            Complex[] d = new Complex[complex.Length / 2];
            for (int k = 0; k < complex.Length / 2; k++)
            {
                e[k] = complex[2 * k];
                d[k] = complex[2 * k + 1];
            }

            Complex[] E = Fast(e);
            Complex[] D = Fast(d);
            for (int k = 0; k < complex.Length / 2; k++)
                D[k] *= Complex.FromPolarCoordinates(1, -2 * Math.PI * k / complex.Length);

            for (int k = 0; k < complex.Length / 2; k++)
            {
                complex2[k] = E[k] + D[k];
                complex2[k + complex.Length / 2] = E[k] - D[k];
            }
            return complex2;
        }
    }
}
