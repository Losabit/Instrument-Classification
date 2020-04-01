namespace SoundAnalyzer.Forms
{
    partial class SplitForm
    {
        /// <summary>
        /// Required designer variable.
        /// </summary>
        private System.ComponentModel.IContainer components = null;

        /// <summary>
        /// Clean up any resources being used.
        /// </summary>
        /// <param name="disposing">true if managed resources should be disposed; otherwise, false.</param>
        protected override void Dispose(bool disposing)
        {
            if (disposing && (components != null))
            {
                components.Dispose();
            }
            base.Dispose(disposing);
        }

        #region Windows Form Designer generated code

        /// <summary>
        /// Required method for Designer support - do not modify
        /// the contents of this method with the code editor.
        /// </summary>
        private void InitializeComponent()
        {
            this.textBox1 = new System.Windows.Forms.TextBox();
            this.label1 = new System.Windows.Forms.Label();
            this.buttonSplit = new System.Windows.Forms.Button();
            this.SuspendLayout();
            // 
            // textBox1
            // 
            this.textBox1.Location = new System.Drawing.Point(391, 161);
            this.textBox1.Name = "textBox1";
            this.textBox1.Size = new System.Drawing.Size(100, 22);
            this.textBox1.TabIndex = 0;
            // 
            // label1
            // 
            this.label1.AutoSize = true;
            this.label1.Location = new System.Drawing.Point(64, 161);
            this.label1.Name = "label1";
            this.label1.Size = new System.Drawing.Size(321, 17);
            this.label1.TabIndex = 1;
            this.label1.Text = "Nombre de Canaux Maximum à import par fichier :";
            // 
            // buttonSplit
            // 
            this.buttonSplit.Location = new System.Drawing.Point(308, 305);
            this.buttonSplit.Name = "buttonSplit";
            this.buttonSplit.Size = new System.Drawing.Size(154, 69);
            this.buttonSplit.TabIndex = 2;
            this.buttonSplit.Text = "Spliter";
            this.buttonSplit.UseVisualStyleBackColor = true;
            this.buttonSplit.Click += new System.EventHandler(this.buttonSplit_Click);
            // 
            // SplitForm
            // 
            this.AutoScaleDimensions = new System.Drawing.SizeF(8F, 16F);
            this.AutoScaleMode = System.Windows.Forms.AutoScaleMode.Font;
            this.ClientSize = new System.Drawing.Size(800, 450);
            this.Controls.Add(this.buttonSplit);
            this.Controls.Add(this.label1);
            this.Controls.Add(this.textBox1);
            this.Name = "SplitForm";
            this.Text = "SplitForm";
            this.ResumeLayout(false);
            this.PerformLayout();

        }

        #endregion

        private System.Windows.Forms.TextBox textBox1;
        private System.Windows.Forms.Label label1;
        private System.Windows.Forms.Button buttonSplit;
    }
}