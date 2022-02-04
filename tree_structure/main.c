/**
 *  Course: DD1338 - Data Structure
 *  Task 15 - Balanced (Binary) Tree Structure 
 *  
 *  Assignment: Write and implement a balanced tree structure using the language of your choice.
 *
 *
 *  Questions:
 *  [1] Accessing a balanced tree has a time-complexity of O(log n) as it it tries to balancing
 *      all paths to be of an equal length.
 *      An unbalanced tree has a worst case time-complexity of O(n), which is equal to a linked
 *      list. If an unbalanced tree is inserted with sorted data, the tree will form a tree which
 *      is equvivalant to a linked list.
 *  
 *  [2] Skip-List, is like skipping class but effective.
 *      A skip-list consists of multiple layers of express lanes.
 *      The lowest layer of the skip-list is a sorted linkedlist.
 *      If the input is 5, it first looks at the first express lane and searches for a node, 
 *      where the following node is larger than the searched value. It then goes a layer lower
 *      and repeats until it reaches the last layer where it can find the specified key.
 *      
 *      (3)  0     3           9
 *      (2)  0     3     6     9
 *      (1)  0 1 2 3 4 5 6 7 8 9 
 *
 *  Language:   C
 *  Author:     Carl (Bae) (Spyan) (Toxic) Chemnitz
 */

#include <stdio.h>
#include <stdlib.h>

#define min(x,y)    ((x < y) ? x : y)

// NOT USED, implementation in future maybe lmao funny noises
union data_t { 
  char              *string;
  int               sint;
  unsigned int      uint;
  long              ilong;
  long long         illong;
  float             decimal;
  double            dec2;;
};

typedef struct  node {
  int           data, key, level;
  struct node   *child[2];
} AANode, *AATree;

#define LEFT        0
#define RIGHT       1
#define left        child[LEFT]
#define right       child[RIGHT]

static AANode       *new_node(int       key,
                              const int data) {
  AATree t          = (AANode*)malloc(sizeof(AANode));
  t->key            = key;
  t->data           = data;
  t->left           = NULL;
  t->right          = NULL;
  t->level          = 1;
  return            t; }

enum                Direction { Right, Left };

static AATree       rotate(AATree           t, 
                           enum Direction   dir) {
  AATree            rl;
  int               a = LEFT, b = RIGHT;
  if(dir == Right)  { a = RIGHT; b = LEFT; }             
  rl                = t->child[a];
  t->child[a]       = rl->child[b];
  rl->child[b]      = t;
  if(dir == Right)  rl->level++;
  return rl; }

static AATree               skew(AATree t) {
  if(t == NULL)             return NULL;
  if(t->left == NULL)       return t;
  if(t->left->level == t->level) 
    return rotate(t, Left);
  return                    t; }

static AATree                   split(AATree t) {
  if(t == NULL)                 return NULL;
  if(t->right == NULL ||
     t->right->right == NULL)   return t;
  if(t->right->right->level == t->level)
    return rotate(t, Right);
  return                        t; }

static AATree                   insert(AATree       t,
                                   int          key,
                                   const int    data) {
  if(t == NULL)             return new_node(key, data);
  else if(key < t->key)     t->left = insert(t->left, key, data);
  else if(key > t->key)     t->right = insert(t->right, key, data);
  t                         = skew(t);
  t                         = split(t);
  return                    t; }

void insert_(AATree *t, int key, const int data) {
  *t = insert(*t, key, data); 
}


static AATree                   delete(AATree   t,
                                       int      key) {
  if(t == NULL)             return NULL;
  if(key < t->key)          t->left  = delete(t->left,  key);
  else if(key > t->key)     t->right = delete(t->right, key);
  else
  if(t->left  == NULL &&
    t->right == NULL)       { free(t); return NULL; }
  if(t->left == NULL)       { AATree    l;
                              l         = t->right;
                              while(l->left != NULL)
                                l       = l->left;
                              t->key    = l->key;
                              t->data   = l->data;
                              t->right   = delete(t->right, l->key); }   
  else                      { AATree    l;
                              l         = t->left;
                              while(l->left != NULL)
                                l       = l->right;
                              t->key    = l->key;
                              t->data   = l->data;
                              t->left   = delete(t->left, l->key);
                            }
  if(t->left != NULL &&
      t->right != NULL)     { int lvl = min(t->left->level, t->right->level) + 1;
    if(lvl < t->level)          { t->level = lvl;
      if(t->right != NULL &&
         lvl < t->right->level) t->right->level = lvl;
                                }
                            }
  t = skew(t);
  t->right = skew(t->right);
  if(t->right != NULL && t->right->right != NULL)
  t->right->right = skew(t->right->right);
  t = split(t);
  t->right = split(t->right); 
  return t;
}

void delete_(AATree* t, int key) { *t = delete(*t, key); }

struct trunk {
	struct trunk *prev;
	char * str;
};
 
void show_trunks(struct trunk *p)
{
	if (!p) return;
	show_trunks(p->prev);
	printf("%s", p->str);
}
 
// this is very haphazzard
void show_tree(AATree root, struct trunk *prev, int is_left)
{
	if (root == NULL) return;
 
	struct trunk this_disp = { prev, "    " };
	char *prev_str = this_disp.str;
	show_tree(root->left, &this_disp, 1);
 
	if (!prev)
		this_disp.str = "---";
	else if (is_left) {
		this_disp.str = ".--";
		prev_str = "   |";
	} else {
		this_disp.str = "`--";
		prev->str = prev_str;
	}
 
	show_trunks(&this_disp);
	printf("%d (%d)\n", root->data, root->key);
 
	if (prev) prev->str = prev_str;
	this_disp.str = "   |";
 
	show_tree(root->right, &this_disp, 0);
	if (!prev) puts("");
}

int main() {
  AATree    aat = NULL;
  insert_(&aat, 0, 5);
  show_tree(aat, 0,0);
  insert_(&aat, 1, 6);
  show_tree(aat, 0,0);
  insert_(&aat, 8, 8);
  show_tree(aat, 0,0);
  insert_(&aat, 3, 6);
  show_tree(aat, 0,0);
  insert_(&aat, 4, 10);
  show_tree(aat, 0,0);
  insert_(&aat, 5, 10);
  show_tree(aat, 0,0);
  insert_(&aat, 10, 10);
  show_tree(aat, 0,0);
  delete_(&aat, 8);
  show_tree(aat, 0,0);
  return 0;
}
