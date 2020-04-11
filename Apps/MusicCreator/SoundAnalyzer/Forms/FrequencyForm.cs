using System;
using System.Collections.Generic;
using System.ComponentModel;
using System.Data;
using System.Diagnostics;
using System.Drawing;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using System.Windows.Forms;

namespace SoundAnalyzer.Forms
{
    public partial class FrequencyForm : Form
    {
        private ListBox listBox;

        public FrequencyForm(ListBox listbox)
        {
            InitializeComponent();
            this.listBox = listbox;
        }

        private void buttonFrequency_Click(object sender, EventArgs e)
        {
            try
            {
                int frequency = 0;
                if (!int.TryParse(textBoxFrequence.Text, out frequency))
                    throw new Exception("La fréquence doit-être un entier");

                if (string.IsNullOrEmpty(textBoxPath.Text))
                    throw new Exception("Donnez un répertoire de sortie");

                Process proc;
                for (int i = 0; i < listBox.SelectedItems.Count; i++)
                {
                    //ffmpeg -i "Lauryn-Hill-Doo-Wop-_That-Thing_-_Official-Video_ - Copie1.wav" -ar 8000 lauryn8000.wav
                    var processInfo = new ProcessStartInfo("ffmpeg", " -i \"" + ((Models.FileImport)listBox.SelectedItems[i]).Path + "\" -ar " + frequency.ToString() + " \"" + textBoxPath.Text + System.IO.Path.DirectorySeparatorChar + ((Models.FileImport)listBox.SelectedItems[i]).File.Replace(".wav", "freq" + frequency.ToString() + ".wav") + "\"")
                    {
                        CreateNoWindow = true,
                        UseShellExecute = false
                    };
                    if ((proc = Process.Start(processInfo)) == null)
                        throw new InvalidOperationException("please install ffmpeg");

                    proc.WaitForExit();
                }
                Close();
            }
            catch (Exception exception)
            {
                MessageBox.Show(exception.Message, "Error", MessageBoxButtons.OK, MessageBoxIcon.Error);
            }
        }

        private void buttonPath_Click(object sender, EventArgs e)
        {
            var openFileDialog = new FolderBrowserDialog();
            var result = openFileDialog.ShowDialog();
            if (result == DialogResult.OK)
                textBoxPath.Text = openFileDialog.SelectedPath;
        }
    }
}
