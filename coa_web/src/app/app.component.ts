import { Component, OnInit } from '@angular/core';
import { FormControl } from '@angular/forms';
import { FileValidators } from 'ngx-file-drag-drop';
import { coa_from_image_vanilla_colors, coa_from_image_all_colors, coa_from_image_reduced_colors } from 'coa_converter_wasm';
import { MatDialog } from '@angular/material/dialog';
import { MessageBoxComponent } from './Dialogs/message-box/message-box.component';


@Component({
  selector: 'app-root',
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.scss']
})
export class AppComponent implements OnInit {
  constructor(
    private dialog: MatDialog
  ) {}

  title = 'coa';
  fileControl = new FormControl([], [FileValidators.required, FileValidators.uniqueFileNames]);
  resolution=100;
  isTitle = false;
  showControls = false;
  result = "";


  delay(n: number){
    return new Promise(function(resolve){
        setTimeout(resolve,n);
    });
}

  async onReducedColors() {
    this.result = "";
    await this.delay(50);
    let file = this.fileControl.value[0] as File;
    let buffer = await file.arrayBuffer();
    let array = new Uint8Array(buffer);
    coa_from_image_reduced_colors(array, this.isTitle, 20, this.resolution).then(result => {
      if(result){
        this.result = result;
      }
      else {
        this.dialog.open(MessageBoxComponent, {
          data: { message: 'A error occurred. Make sure to submit a valid image!' },
        });
      }
    }).catch(() => {
      this.dialog.open(MessageBoxComponent, {
        data: { message: 'Something went wrong! Try reloading the page!' },
      });
    });   
  }

  async onAllColors() {
    this.result = "";
    let file = this.fileControl.value[0] as File;
    let buffer = await file.arrayBuffer();
    let array = new Uint8Array(buffer);
    coa_from_image_all_colors(array, this.isTitle, this.resolution).then(result => {
      if(result){
        this.result = result;
      }
      else {
        this.dialog.open(MessageBoxComponent, {
          data: { message: 'A error occurred. Make sure to submit a valid image!' },
        });
      }
    }).catch(() => {
      this.dialog.open(MessageBoxComponent, {
        data: { message: 'Something went wrong! Try reloading the page!' },
      });
    });   
  }

  async onVanillaColors() {
    this.result = "";
    let file = this.fileControl.value[0] as File;
    let buffer = await file.arrayBuffer();
    let array = new Uint8Array(buffer);
    coa_from_image_vanilla_colors(array, this.isTitle, this.resolution).then(result => {
      if(result){
        this.result = result;
      }
      else {
        this.dialog.open(MessageBoxComponent, {
          data: { message: 'A error occurred. Make sure to submit a valid image!' },
        });
      }
    }).catch(() => {
      this.dialog.open(MessageBoxComponent, {
        data: { message: 'Something went wrong! Try reloading the page!' },
      });
    });   
  }

  onValueChange() {
    this.showControls = this.fileControl.value.length != 0;
  }

  ngOnInit(): void {
      this.fileControl.valueChanges.subscribe(() => this.onValueChange());
  }
}
