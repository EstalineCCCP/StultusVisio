//  This file is part of StultusVisio.
//  Licença no fim. 
//  Licence at the end.
use std::fs;

/// Confecciona um script padrão ou retorna de um arquivo externo indicado pela âncora `.script`.

/// Cria um script em JavaScript para compilação, podendo ser apontado por `.script` como 
/// arquivo personalizado ou, caso inexista o apontamento, entrega um padrão.
///
/// O script padrão define comportamentos de atalhos, transição e marcadores.
pub fn generate_script(script_path: Option<String>) -> String {
    match script_path {
        Some(script_path) => format!("<script>{}</script>", fs::read_to_string(script_path).expect("Verifique o javascript informado")),
        None => r#"
        <script>
var slides = document.querySelectorAll('.slide');
var currentslideIndex = 0;
var n = 0;

const popup = document.getElementById("popup");
const popupText = document.getElementById("conteudo-popup");

function popUpShow(){
    popupText.innerHTML = `
                 <h1>Pressione <em>"t"</em><br> para fechar ajuda</h1>
                 <h2>Slide ${currentslideIndex+1} de ${slides.length}</h2>
                 <table>
                 <tr>
                   <th>Comando</th><th>Tecla</th>
                 </tr>
                 <tr>
                   <td>Próximo</td><td>j</td>
                 </tr>
                 <tr>
                   <td>Anterior</td><td>k</td>
                 </tr>
                 <tr>
                   <td>Topo</td><td>gg</td>
                 <tr>
                   <td>Final</td><td>G</td>
                 </tr>
                 <tr>
                   <td>Modo impressão</td><td>p</td>
                 </tr>

                 </table>
               `;
}
popUpShow();

function Printar() {
  for (var j = 0; j < slides.length; j++) {
      slides[j].style.display= 'block';
  }
};

document.addEventListener('keydown', function(event) {
  if (event.key === 'ArrowRight' || event.key === 'j') {
    if (currentslideIndex < slides.length - 1){
      var contador = parseInt(n);
      if (contador === 0) {contador = contador + 1}
      else if (contador > slides.length) {contador = 0};
      currentslideIndex = currentslideIndex + contador;
      n = 0;
    }
  }
  else if (event.key === 'ArrowLeft' || event.key === 'k'){
    if (currentslideIndex > 0) {
     var contador = parseInt(n);
     if (contador === 0) {contador = contador + 1};
     currentslideIndex = currentslideIndex - contador;
     if (currentslideIndex < 0 ){currentslideIndex = 0};
     n = 0;
    }
  }
  else if (event.key === 'p') {
    Printar();
    return;
  }
  else if (event.key === 't') {
    if (popup.style.display === 'none') {popup.style.display = "block"}
    else {popup.style.display = "none"};
    popUpShow()
  }
  else if (typeof event.key === 'string' && event.key >= '0' && event.key <= '9'){
    if (n === 0) {n = `${event.key}`}else{n = `${n}${event.key}`}
  }

  else if (event.key === 'g'){
    document.addEventListener('keydown', function(event) {
       if (event.key === 'g') {currentslideIndex = 0};
    })
  }
  
  else if (event.key === 'G') { currentslideIndex = slides.length - 1}

  else if (event.key === 'm'){toggleMovement()}

  else if (isMoving && event.key === 'ArrowUp' 
    || event.key === 'ArrowLeft' 
    || event.key === 'ArrowDown' 
    || event.key === 'ArrowRight') {moveCircle(event.key)}

  else if (event.key === 'x') {resizeMarcador()}
  
  else if (event.key === '-') {alterarTamanhoFonte()}

  else {return};

  for (var i = 0; i < slides.length; i++) {
    if (i === currentslideIndex) {
      slides[i].style.display = 'flex';
    } else {
      slides[i].style.display = 'none';
    }
  }
});

const circle = document.getElementById('marcador');
var tamanhoMarcador = 1;
let circleTop = 0;
let circleLeft = 0;
let isMoving = false;

function updateCirclePosition() {
  circle.style.top = circleTop + 'px';
  circle.style.left = circleLeft + 'px';
}

function resizeMarcador() {
  if (tamanhoMarcador > 250) {tamanhoMarcador = 1} 
  else {tamanhoMarcador = tamanhoMarcador + 5};
  circle.style.width = tamanhoMarcador + 'px';
  circle.style.height = tamanhoMarcador + 'px';
}

document.addEventListener('mousemove', function(event) {
  const x = event.clientX;
  const y = event.clientY;
  marcador.style.transform = `translate(${x}px, ${y}px)`;
});

function moveCircle(direction) {
  const step = 10; 
  switch (direction) {
    case 'ArrowDown':
      circleTop += step;
      break;
    case 'ArrowUp':
      circleTop -= step;
      break;
    case 'ArrowLeft':
      circleLeft -= step;
      break;
    case 'ArrowRight':
      circleLeft += step;
      break;
  }

  updateCirclePosition();
}

function toggleMovement() {
  isMoving = !isMoving;
  if (isMoving) {
    circle.style.display = 'block';
  } else {
    circle.style.display = 'none';
  }
}
function alterarTamanhoFonte() {
    var elementosSlide = document.querySelectorAll(".slide");
    if (elementoSlide) {
        var novoTamanhoFonte = "16px";
        elementoSlide.style.fontSize = novoTamanhoFonte;
    } else {
        console.error("Elemento 'slide' não encontrado.");
    }
}

// Executa uma tecl 'k' para iniciar layout correta
document.addEventListener('DOMContentLoaded', function() {
  const event = new KeyboardEvent('keydown', {
    key: 'k',
    keyCode: 75,
    code: 'KeyK',
    which: 75,
    shiftKey: false, 
    ctrlKey: false, 
    altKey: false, 
    metaKey: false, 
  });
  
  document.dispatchEvent(event);
});
</script>"#.to_string(),
    }
}

//    StultusVisio is free software: you can redistribute it and/or modify
//    it under the terms of the GNU General Public License as published by
//    the Free Software Foundation, either version 3 of the License, or
//    (at your option) any later version.
//
//    StultusVisio is distributed in the hope that it will be useful,
//    but WITHOUT ANY WARRANTY; without even the implied warranty of
//    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//    GNU General Public License for more details.
//
//    You should have received a copy of the GNU General Public License
//    along with StultusVisio.  If not, see <https://www.gnu.org/licenses/>6.
//    Jefferson T. @ 2023. Telegram: StalinCCCP

